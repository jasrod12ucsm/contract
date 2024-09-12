use bod_models::schemas::config::card_plan::models::card_plan_with_id::CardPlanWithId;
use bod_models::schemas::config::company::models::company_with_id::CompanyWithId;
use bson::oid::ObjectId;
use bson::{doc, DateTime};
use chrono::Utc;
use common::utils::ntex_private::repository::public_repository::{
    AbstractRepository, PublicRepository, Repository,
};
use cron::Schedule;
use futures::{StreamExt, TryStreamExt};
use mongodb::options::{UpdateOneModel, WriteModel};
use utils::domain::models::culqi_create_subscription::CulqiCreateSubscriptionBuilder;
use utils::domain::models::culqi_create_subscription_response::CulqiCreateSubscriptionResponse;
use utils::domain::models::culqi_get_subscription_response::CulqiGetSubscriptionResponse;
use utils::infraestructure::repositories::card_plan_repositoy;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use std::{str::FromStr, sync::Arc};
use tokio::time::{sleep, Duration};
use utils::{
    domain::{
        datasources::culqi_datasource_trait::CulqiDataSourceTrait,
        repositories::culqi_subscription_repository_trait::CulqiSubscriptionRepositoryTrait,
    },
    infraestructure::{
        datasources::culqi_datasource::CulqiDatasource,
        repositories::{
            company_repository::CompanyRepository,
            culqi_subscription_repository::CulqiSubscriptionRepository,
        },
    },
};

use rayon::prelude::*;

pub mod utils;

async fn perform_cron_job(
    public_repository: &PublicRepository,
    culqi_subscription_repository: Arc<CulqiSubscriptionRepository>,
    now: DateTime,
) -> Result<(), PerformCronJobError> {
    println!("Verificando y actualizando suscripciones...");

    let company_repository = Arc::new(
        public_repository
            .get_repository::<CompanyRepository>()
            .await
            .map_err(|e| {
                eprintln!("Error getting CompanyRepository: {:?}", e);
                PerformCronJobError::new()
            })?,
    );
    let card_plan_repositoy = Arc::new(
        public_repository
            .get_repository::<card_plan_repositoy::CardPlanRepository>()
            .await
            .map_err(|e| {
                eprintln!("Error getting CardPlanRepository: {:?}", e);
                PerformCronJobError::new()
            })?,
    );

    let mut card_plan = card_plan_repositoy
        .find(doc! {"noDeleted":true})
        .await
        .map_err(|e| {
            eprintln!("Error finding card plans: {:?}", e);
            PerformCronJobError::new()
        })?;

    let order_map = build_order_map(&mut card_plan).await?;

    let perform_cron_error = Arc::new(Mutex::new(PerformCronJobError::new_initialized()));

    let companies = company_repository
        .find(doc! {})
        .await
        .map_err(|e| {
            eprintln!("Error finding companies: {:?}", e);
            PerformCronJobError::new()
        })?;
    
    let vec_companies: Vec<CompanyWithId> = companies
        .try_collect()
        .await
        .map_err(|e| {
            eprintln!("Error collecting companies: {:?}", e);
            PerformCronJobError::new()
        })?;

    let chunks: Vec<Vec<CompanyWithId>> = vec_companies
        .chunks(10)
        .map(|chunk| chunk.to_vec())
        .collect();

    let futures: Vec<_> = chunks
        .par_iter()
        .map(|chunk| {
            let culqi_subscription_repository = Arc::clone(&culqi_subscription_repository);
            let card_plan_repositoy = Arc::clone(&card_plan_repositoy);
            let company_repository = Arc::clone(&company_repository);
            let perform_cron_error = Arc::clone(&perform_cron_error);
            let chunk = chunk.clone();
            let order_map_clone = order_map.clone();
            tokio::spawn(async move {
                process_chunk(
                    chunk,
                    culqi_subscription_repository,
                    card_plan_repositoy,
                    company_repository,
                    perform_cron_error,
                    order_map_clone,
                    now,
                )
                .await;
            })
        })
        .collect();

    for future in futures {
        future.await.unwrap();
    }

    if perform_cron_error.lock().unwrap().indexes_not_actualized.len() > 0 {
        return Err(perform_cron_error.lock().unwrap().clone());
    }

    Ok(())
}

async fn build_order_map(
    card_plan: &mut mongodb::Cursor<CardPlanWithId>,
) -> Result<HashMap<i32, HashMap<i32, (String, Option<DateTime>)>>, PerformCronJobError> {
    let mut order_map = HashMap::new();
    while let Some(card_plan_value) = card_plan.next().await {
        let card_plan_value = card_plan_value.map_err(|e| {
            eprintln!("Error getting card plan value: {:?}", e);
            PerformCronJobError::new()
        })?;
        let mut card_hash = HashMap::new();
        for value in card_plan_value.restaurants_data.clone().into_iter() {
            card_hash.insert(
                value.num_restaurants,
                (value.plan_token, card_plan_value.price_actualized_date),
            );
        }
        order_map.insert(card_plan_value.render.order, card_hash);
    }
    Ok(order_map)
}

async fn process_chunk(
    chunk: Vec<CompanyWithId>,
    culqi_subscription_repository: Arc<CulqiSubscriptionRepository>,
    card_plan_repositoy: Arc<card_plan_repositoy::CardPlanRepository>,
    company_repository: Arc<CompanyRepository>,
    perform_cron_error: Arc<Mutex<PerformCronJobError>>,
    order_map: HashMap<i32, HashMap<i32, (String, Option<DateTime>)>>,
    now: DateTime,
) {
    let mut id_and_subscription: Vec<IdAndSubscription> = Vec::new();
    for company in chunk {
        let subscription_id = &company.sensible.subscription;
        let subscription = culqi_subscription_repository
            .get_subscription(subscription_id.as_str())
            .await;
        if subscription.is_err() {
            if let Some(card_id) = company
                .sensible
                .credit_cards
                .iter()
                .find(|card| card.is_used_card)
                .map(|card| &card.token)
            {
                if let Some((plan, order)) = get_card_plan_token(&company, &card_plan_repositoy).await {
                    if let Some((global_plan_token, global_plan_date)) =
                        order_map.get(&order).and_then(|map| map.get(&company.quantity_restaurant))
                    {
                        if plan == *global_plan_token {
                            if let Some(date) = global_plan_date {
                                if now.timestamp_millis() > date.timestamp_millis() {
                                    if let Ok(actual_subscription) = culqi_subscription_repository
                                        .get_subscription(subscription_id.as_str())
                                        .await
                                    {
                                        if should_update_subscription(&actual_subscription, now) {
                                            if actual_subscription.plan.plan_id != plan {
                                                if let Ok(subscription_new_value) = create_new_subscription(
                                                    &culqi_subscription_repository,
                                                    card_id,
                                                    &plan,
                                                )
                                                .await
                                                {
                                                    cancel_old_subscription(
                                                        &culqi_subscription_repository,
                                                        &actual_subscription.id,
                                                    )
                                                    .await;
                                                    id_and_subscription.push(IdAndSubscription {
                                                        id: company.id,
                                                        subscription: subscription_new_value.id,
                                                    });
                                                }
                                            } else {
                                                perform_cron_error.lock().unwrap().indexes_not_actualized.push(company.id.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    update_companies(&company_repository, id_and_subscription).await;
}

async fn get_card_plan_token(
    company: &CompanyWithId,
    card_plan_repositoy: &Arc<card_plan_repositoy::CardPlanRepository>,
) -> Option<(String, i32)> {
    card_plan_repositoy
        .find_one(doc! {"_id":&company.card_plan,"noDeleted":true}, None)
        .await
        .ok()
        .flatten()
        .and_then(|value_principal| {
            value_principal
                .restaurants_data
                .into_iter()
                .find(|value| value.num_restaurants == company.quantity_restaurant)
                .map(|value| (value.plan_token, value_principal.render.order))
        })
}

fn should_update_subscription(actual_subscription: &CulqiGetSubscriptionResponse, now: DateTime) -> bool {
    now.timestamp_millis() + 3 * 60 * 60 * 1000 >= actual_subscription.next_billing_date
}

async fn create_new_subscription(
    culqi_subscription_repository: &Arc<CulqiSubscriptionRepository>,
    card_id: &str,
    plan: &str,
) -> Result<CulqiCreateSubscriptionResponse, ()> {
    let subsciption_new = CulqiCreateSubscriptionBuilder::new()
        .card_id(card_id.to_string())
        .plan_id(plan.to_string())
        .build();
    culqi_subscription_repository
        .create_subscription(subsciption_new)
        .await
        .map_err(|e| {
            eprintln!("Error creating new subscription: {:?}", e);
            ()
        })
}

async fn cancel_old_subscription(
    culqi_subscription_repository: &Arc<CulqiSubscriptionRepository>,
    subscription_id: &str,
) {
    if let Ok(_) = culqi_subscription_repository
        .cancel_subscription(subscription_id)
        .await
    {
        println!("Se canceló la suscripción");
    } else {
        println!("Error al cancelar la suscripción");
    }
}

async fn update_companies(
    company_repository: &Arc<CompanyRepository>,
    id_and_subscription: Vec<IdAndSubscription>,
) {
    let documents: Vec<WriteModel> = id_and_subscription
        .into_iter()
        .map(|value| {
            WriteModel::UpdateOne(
                UpdateOneModel::builder()
                    .filter(doc! {"_id":value.id})
                    .update(doc! {"$set":{
                        "sensible.subscription":value.subscription
                    }})
                    .namespace(company_repository.get_collection().namespace())
                    .upsert(false)
                    .build(),
            )
        })
        .collect();
    let _ = company_repository.get_client().bulk_write(documents).await;
}

#[tokio::main]
async fn main() {
    let public_repository = PublicRepository::connect()
        .await
        .expect("Error connecting to PublicRepository");
    let datasource = Arc::new(CulqiDatasource::new());
    let subscription_repository = Arc::new(CulqiSubscriptionRepository::new(datasource));

    let schedule = Schedule::from_str("0 */2 * * *").unwrap();

    let mut next_run = Instant::now();
    loop {
        let now = Utc::now().naive_utc();

        if let Some(next) = schedule.upcoming(chrono::Utc).next() {
            next_run = Instant::now()
                + Duration::from_secs((next.timestamp() - now.and_utc().timestamp()) as u64);
        }

        let delay = next_run
            .duration_since(Instant::now())
            .max(Duration::from_secs(0));
        sleep(delay).await;

        perform_cron_job(
            &public_repository,
            Arc::clone(&subscription_repository),
            DateTime::now(),
        )
        .await
        .unwrap();

        next_run = Instant::now() + Duration::from_secs(7200);
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PerformCronJobError {
    was_initialized: bool,
    indexes_not_actualized: Vec<String>,
}

impl PerformCronJobError {
    fn new() -> Self {
        Self {
            was_initialized: false,
            indexes_not_actualized: vec![],
        }
    }

    fn new_initialized() -> Self {
        Self {
            was_initialized: true,
            indexes_not_actualized: vec![],
        }
    }
}

struct IdAndSubscription {
    id: ObjectId,
    subscription: String,
}