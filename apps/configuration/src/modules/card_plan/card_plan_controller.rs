use crate::{
    modules::card_plan::{
        data::update_cardplan_dto::UpdateCardPlanDto,
        models::card_plan_projection::CardPlanProjection,
    },
    utils::repositories::card_plan_repositoy::CardPlanRepository,
};
use bod_models::schemas::config::card_plan::{
    card_plan_error::CardPlanError, models::card_plan_with_id::CardPlanWithId,
};
use bson::{doc, oid::ObjectId};
use common::{
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};
use futures::StreamExt;
use mongodb::Collection;
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
    let card_plan_projection_collection: Collection<CardPlanProjection> =
        card_plan_repository.construct_new_collection();
    let mut card_plans = card_plan_repository
        .find_generic(doc! {"noDeleted":true}, &card_plan_projection_collection)
        .sort(doc! {"order": 1})
        .projection(doc! {
            "order": 0,
            "isActive": 0,
            "updatedAt": 0,
            "_id": 0
        })
        .await
        .map_err(|_| CardPlanError::GetCardPlansError("Data failure"))?;
    let mut card_plans_vector = vec![];
    while let Some(card_plan) = card_plans.next().await {
        if card_plan.is_err() {
            return Err(CardPlanError::GetCardPlansError(
                "One or more card plans not found",
            ));
        }
        let card_plan = card_plan.unwrap();
        card_plans_vector.push(card_plan);
    }
    Ok(JsonAdvanced(card_plans_vector))
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

    let mut update_doc = doc! {};
    if let Some(button) = &update_dto.button {
        update_doc.insert("button", button);
    }

    if let Some(price) = update_dto.price {
        update_doc.insert("price", price);
    }
    if let Some(shape) = &update_dto.shape {
        update_doc.insert("shape", shape);
    }
    if let Some(items) = &update_dto.items {
        update_doc.insert("items", bson::to_bson(items).unwrap());
    }

    let document_to_update = doc! {
        "$set": update_doc,
    };

    let card_plan_updated = card_plan_repository
        .find_one_and_update(doc! {"_id": card_plan_id}, document_to_update)
        .await
        .map_err(|_| CardPlanError::UpdateCardPlanError("can't update plan"))?
        .ok_or_else(|| CardPlanError::UpdateCardPlanError("Card plan not found"))?;
    Ok(JsonAdvanced(card_plan_updated))
}
