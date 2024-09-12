use std::rc::Rc;

use crate::{
    modules::card_plan::{
        data::update_cardplan_dto::UpdateCardPlanDto,
        models::card_plan_projection::CardPlanProjection,
    },
    utils::{
        domain::{
            datasources::culqi_datasource_trait::CulqiDataSourceTrait,
            models::create_culqi_plan::{
                CreateCulqiPlan, Currency, InitialCycles, IntervalUnitTime,
            },
            repositories::culqi_plan_repository_trait::CulqiPlanRepositoryTrait,
        },
        infraestructure::{
            datasources::culqi_datasource::CulqiDatasource,
            repositories::{
                card_plan_repositoy::CardPlanRepository, culqi_plan_repository::CulqiPlanRepository,
            },
        },
    },
};
use bod_models::schemas::config::card_plan::{
    card_plan_error::CardPlanError, models::card_plan_with_id::CardPlanWithId,
};
use bson::{doc, oid::ObjectId, DateTime};
use common::{
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};

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

    let card_plan_id = ObjectId::parse_str(path.id())
        .map_err(|_| CardPlanError::UpdateCardPlanError("error parsing id"))?;

    let qulqi_datasource = CulqiDatasource::new();
    let culqi_plan_repository = CulqiPlanRepository::new(&qulqi_datasource);

    // Obtener el documento actual de card_plan
    let current_card_plan = card_plan_repository
        .find_one(doc! {"_id": card_plan_id}, None)
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("can't find card plan"))?
        .ok_or_else(|| CardPlanError::UpdateCardPlanError("Card plan not found"))?;

    let mut update_doc = doc! {};
    let mut new_tokens = vec![];
    let mut render_doc = doc! {};
    let mut was_initialized_date_updated = Rc::new(false);
    if let Some(value) = &update_dto.render {
        if let Some(button) = &value.button {
            render_doc.insert("button", button);
        }
        if let Some(price) = &value.price {
            update_doc.insert("priceActualizedDate", DateTime::now());
            let was_initialized = Rc::make_mut(&mut was_initialized_date_updated);
            *was_initialized = true;
            render_doc.insert("price", price);

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

            // Usar pricePerRestaurant del update_dto o del documento actual
            let price_per_restaurant = update_dto
                .price_per_restaurant
                .unwrap_or_else(|| current_card_plan.price_per_restaurant);

            let restaurants_data: Vec<bson::Document> = (1..=100)
                .map(|num_restaurants| {
                    let total_price = price + price_per_restaurant * (num_restaurants - 1);
                    doc! {
                        "totalPrice": total_price,
                        "planToken": new_tokens[num_restaurants as usize - 1].clone(),
                        "numRestaurants": num_restaurants
                    }
                })
                .collect();

            update_doc.insert("restaurantsData", bson::to_bson(&restaurants_data).unwrap());
        } else if let Some(price_per_restaurant) = &update_dto.price_per_restaurant {
            // Usar el price inicial del documento actual
            let initial_price = current_card_plan.render.price;
            let was_initialized = Rc::make_mut(&mut was_initialized_date_updated);
            if *was_initialized == false {
                update_doc.insert("priceActualizedDate", DateTime::now());
                *was_initialized = true;
            }
            // Crear nuevos planes en Culqi para cada restaurante usando el price inicial y el nuevo pricePerRestaurant
            for num_restaurants in 1..=100 {
                let plan = CreateCulqiPlan::builder()
                    .name(format!("Plan {} Restaurantes", num_restaurants))
                    .short_name(format!("Plan{}", num_restaurants))
                    .description(format!("Plan para {} restaurantes", num_restaurants))
                    .amount(initial_price + price_per_restaurant * (num_restaurants - 1))
                    .currency(Currency::PEN)
                    .interval_unit_time(IntervalUnitTime::Monthly)
                    .interval_count(1)
                    .initial_cycles(InitialCycles {
                        count: 1,
                        has_initial_charge: true,
                        amount: initial_price + price_per_restaurant * (num_restaurants - 1),
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

            // Recalcular los planes usando el price inicial y el nuevo pricePerRestaurant
            let restaurants_data: Vec<bson::Document> = (1..=100)
                .map(|num_restaurants| {
                    let total_price = initial_price + price_per_restaurant * (num_restaurants - 1);
                    doc! {
                        "totalPrice": total_price,
                        "planToken": new_tokens[num_restaurants as usize - 1].clone(),
                        "numRestaurants": num_restaurants
                    }
                })
                .collect();

            update_doc.insert("restaurantsData", bson::to_bson(&restaurants_data).unwrap());
        }

        if let Some(shape) = &value.shape {
            render_doc.insert("shape", shape);
        }
        if let Some(items) = &value.items {
            render_doc.insert("items", bson::to_bson(items).unwrap());
        }
    }
    if render_doc.len() > 0 {
        update_doc.insert("render", render_doc);
    }
    if let Some(price_per_restaurant) = &update_dto.price_per_restaurant {
        update_doc.insert("pricePerRestaurant", price_per_restaurant);
        let was_initialized = Rc::make_mut(&mut was_initialized_date_updated);
        if *was_initialized == false {
            update_doc.insert("priceActualizedDate", DateTime::now());
            *was_initialized = true;
        }
    }

    let document_to_update = doc! {
        "$set": update_doc,
    };

    let card_plan_updated = {
        let card_plan_repository = card_plan_repository;
        card_plan_repository
            .find_one_and_update(doc! {"_id": card_plan_id}, document_to_update)
            .await
            .map_err(|_| CardPlanError::UpdateCardPlanError("can't update plan"))?
            .ok_or_else(|| CardPlanError::UpdateCardPlanError("Card plan not found"))?
    };

    Ok(JsonAdvanced(card_plan_updated))
}
