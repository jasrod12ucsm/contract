use crate::{
    modules::card_plan::{
        data::update_cardplan_dto::UpdateCardPlanDto,
        models::card_plan_projection::CardPlanProjection,
    },
    utils::{
        domain::{
            datasources::culqi_datasource_trait::CulqiDataSourceTrait,
            models::create_culqi_plan::{CreateCulqiPlan, Currency, InitialCycles, IntervalUnitTime},
        },
        infraestructure::{
            datasources::culqi_datasource::CulqiDatasource,
            repositories::{card_plan_repositoy::CardPlanRepository, company_repository},
        },
    },
};
use bod_models::schemas::config::{card_plan::{
    card_plan_error::CardPlanError, models::card_plan_with_id::CardPlanWithId,
}, company::company::Company};
use bson::{doc, oid::ObjectId};
use common::{
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};

use futures::StreamExt;
use ntex::web::{
    self,
    types::{Path, State},
};

#[web::get("get_all")]
pub async fn get_all_card_plans(
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<CardPlanProjection>>, CardPlanError> {
    let card_plan_repository: CardPlanRepository = repo
        .get_repository::<CardPlanRepository>()
        .await
        .map_err(|_| CardPlanError::GetCardPlansError("Internal serve error"))?;
    let card_plans = card_plan_repository
        .get_card_plan_projection_collection()
        .await?;
    Ok(JsonAdvanced(card_plans))
}

#[web::put("update/{id}")]
pub async fn update_card_plan(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
    update_dto: JsonAdvanced<UpdateCardPlanDto>,
) -> Result<JsonAdvanced<CardPlanWithId>, CardPlanError> {
    let card_plan_repository: CardPlanRepository = repo
        .get_repository::<CardPlanRepository>()
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("update card plan error"))?;
    let company_repository: company_repository::CompanyRepository = repo
        .get_repository::<company_repository::CompanyRepository>()
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("update card plan error"))?;
    let culqi_plan_repository: CulqiPlanRepository = repo
        .get_repository::<CulqiPlanRepository>()
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("update card plan error"))?;

    let card_plan_id = ObjectId::parse_str(path.id())
        .map_err(|_| CardPlanError::UpdateCardPlanError("error parsing id"))?;
    let mut update_doc = doc! {};
    let mut new_tokens = vec![];

    if let Some(value) = &update_dto.render {
        if let Some(button) = &value.button {
            update_doc.insert("button", button);
        }

        if let Some(price) = &value.price {
            update_doc.insert("price", price);

            // Crear nuevos planes en Culqi para cada restaurante
            for num_restaurants in 1..=100 {
                let plan = CreateCulqiPlan::builder()
                    .name(format!("Plan {} Restaurantes", num_restaurants))
                    .short_name(format!("Plan{}", num_restaurants))
                    .description(format!("Plan para {} restaurantes", num_restaurants))
                    .amount(price * num_restaurants)
                    .currency(Currency::PEN)
                    .interval_unit_time(IntervalUnitTime::Monthly)
                    .interval_count(1)
                    .initial_cycles(InitialCycles {
                        count: 1,
                        has_initial_charge: true,
                        amount: price * num_restaurants,
                        interval_unit_time: IntervalUnitTime::Monthly,
                    })
                    .build()
                    .map_err(|_| {
                        CardPlanError::UpdateCardPlanError("Failed to build CreateCulqiPlan")
                    })?;

                let response = culqi_plan_repository.create_plan(plan).await.map_err(|_| {
                    CardPlanError::UpdateCardPlanError("Failed to create plan in Culqi")
                })?;

                new_tokens.push(response.id);
            }

            let restaurants_data: Vec<bson::Document> = (1..=100)
                .map(|num_restaurants| {
                    doc! {
                        "totalPrice": price * num_restaurants,
                        "planToken": new_tokens[num_restaurants as usize - 1].clone(),
                        "numRestaurants": num_restaurants
                    }
                })
                .collect();

            update_doc.insert("restaurantsData", bson::to_bson(&restaurants_data).unwrap());
        }

        if let Some(shape) = &value.shape {
            update_doc.insert("shape", shape);
        }
        if let Some(items) = &value.items {
            update_doc.insert("items", bson::to_bson(items).unwrap());
        }
    }
    if let Some(price_per_restaurant) = &update_dto.price_per_restaurant {
        update_doc.insert("pricePerRestaurant", price_per_restaurant);
    }

    let document_to_update = doc! {
        "$set": update_doc,
    };

    let card_plan_updated = card_plan_repository
        .find_one_and_update(doc! {"_id": card_plan_id}, document_to_update)
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("can't update plan"))?
        .ok_or_else(|| CardPlanError::UpdateCardPlanError("Card plan not found"))?;

    // Obtener todas las compañías activas y no borradas
    let active_companies = company_repository
        .find(doc! {})
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("Failed to fetch active companies"))?;
    let companies=vec![];
    while let Some(result) = active_companies.next().await {
        match result {
            Ok(document) => {
                companies.push(document);
            }
            Err(_) => {
                return Err(CardPlanError::UpdateCardPlanError(
                    "Failed to fetch active companies",
                ))
            }
        }
    }

    // Cancelar suscripciones existentes para cada compañía
    for company in companies {
        let num_restaurants = company.;
        let subscription_id = company.subscription_id;

        // Cancelar la suscripción existente
        let cancel_response = company_repository
            .cancel_subscription(&subscription_id)
            .await
            .map_err(|_| CardPlanError::UpdateCardPlanError("Failed to cancel subscription"))?;

        // Aquí puedes agregar lógica adicional si es necesario
    }

    Ok(JsonAdvanced(card_plan_updated))
}