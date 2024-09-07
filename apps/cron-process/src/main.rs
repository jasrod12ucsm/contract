use bod_models::schemas::config::company::models::company_with_id::CompanyWithId;
use bson::doc;
use chrono::Utc;
use common::utils::ntex_private::repository::public_repository::{
    AbstractRepository, PublicRepository,
};
use cron::Schedule;
use futures::StreamExt;
use utils::domain::models::culqi_create_subscription::CulqiCreateSubscriptionBuilder;
use utils::infraestructure::repositories::card_plan_repositoy;

use std::time::Instant;
use std::{str::FromStr, sync::Arc};
use tokio::{
    task,
    time::{sleep, Duration},
};
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
) -> Result<(), PermformCronJobError> {
    println!("Verificando y actualizando suscripciones...");

    // Aquí es donde implementas la lógica de verificación y actualización de suscripciones
    let company_repository = public_repository
        .get_repository::<CompanyRepository>()
        .await
        .map_err(|_| PermformCronJobError {
            was_initialized: false,
            indexes_not_actualized: vec![],
        })?;
    let card_plan_repositoy = Arc::new(
        public_repository
            .get_repository::<card_plan_repositoy::CardPlanRepository>()
            .await
            .map_err(|_| PermformCronJobError {
                was_initialized: false,
                indexes_not_actualized: vec![],
            })?,
    );
    //obten todas las compañias
    let mut companies =
        company_repository
            .find(doc! {})
            .await
            .map_err(|_| PermformCronJobError {
                was_initialized: false,
                indexes_not_actualized: vec![],
            })?;

    let mut vec_companies = Vec::new();
    while let Some(company) = companies.next().await {
        let company = company.map_err(|_| PermformCronJobError {
            was_initialized: false,
            indexes_not_actualized: vec![],
        })?;
        vec_companies.push(company);
    }

    // Dividir las compañías en fragmentos y procesarlas en paralelo
    let chunks: Vec<Vec<CompanyWithId>> = vec_companies
        .chunks(10)
        .map(|chunk| chunk.to_vec())
        .collect();
    let futures: Vec<_> = chunks
        .par_iter()
        .map(|chunk| {
            let culqi_subscription_repository = Arc::clone(&culqi_subscription_repository);
            let chunk = chunk.clone();
            task::spawn(async move {
                for company in chunk {
                    let subscription_id = &company.sensible.subscription;
                    let subscription = culqi_subscription_repository
                        .get_subscription(subscription_id.as_str())
                        .await;
                    let card_plan_repository_spawn = Arc::clone(&card_plan_repositoy);
                    // si no se obtiene la subscripcion crea una nueva con la carta principal
                    if subscription.is_err() {
                        //trae la carta principal
                        let card_id = &company
                            .sensible
                            .credit_cards
                            .into_iter()
                            .filter(|card| card.is_used_card)
                            .next()
                            .unwrap()
                            .token;
                        let card_plan_token = {
                            let card_plan = card_plan_repository_spawn
                                .find_one(doc! {"_id":&company.card_plan,"noDeleted":true}, None)
                                .await
                                .unwrap();
                            if let Some(value) = card_plan {
                                let value = value
                                    .restaurants_data
                                    .into_iter()
                                    .filter(|value| {
                                        value.num_restaurants == (&company).quantity_restaurant
                                    })
                                    .next();
                                if let Some(value) = value {
                                    Some(value.plan_token)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        };
                        if let Some(plan) = card_plan_token {
                            let subsciption_new = CulqiCreateSubscriptionBuilder::new()
                                .card_id(*card_id)
                                .plan_id(plan)
                                .build();
                        }

                        //usamos culqi subscription para ponerlo en un vector de lista de actualizaciones
                    }
                }
            })
        })
        .collect();

    // Esperar a que todas las tareas se completen
    for future in futures {
        future.await.unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Conectar al repositorio
    let public_repository = PublicRepository::connect()
        .await
        .map_err(|_| panic!("Error connecting to PublicRepository"))
        .unwrap();
    let datasource = Arc::new(CulqiDatasource::new());
    let subscription_repository = Arc::new(CulqiSubscriptionRepository::new(datasource));

    // Definir la programación del cron job (ejemplo: cada 2 horas)
    let schedule = Schedule::from_str("0 */2 * * *").unwrap(); // Cada 2 horas

    let mut next_run = Instant::now();
    loop {
        let now = Utc::now().naive_utc();

        // Calcular el próximo tiempo de ejecución basado en el cronograma
        if let Some(next) = schedule.upcoming(chrono::Utc).next() {
            next_run = Instant::now()
                + Duration::from_secs((next.timestamp() - now.and_utc().timestamp()) as u64);
        }

        // Esperar hasta el próximo tiempo de ejecución
        let delay = next_run
            .duration_since(Instant::now())
            .max(Duration::from_secs(0));
        sleep(delay).await;

        // Ejecutar el cron job
        perform_cron_job(&public_repository, Arc::clone(&subscription_repository))
            .await
            .unwrap();

        // Recalcular el próximo tiempo de ejecución
        next_run = Instant::now() + Duration::from_secs(7200); // 2 horas
    }
}

#[derive(Debug)]
struct PermformCronJobError {
    was_initialized: bool,
    indexes_not_actualized: Vec<String>,
}

struct IdAndSubscription {
    id: String,
    subscription: String,
}
