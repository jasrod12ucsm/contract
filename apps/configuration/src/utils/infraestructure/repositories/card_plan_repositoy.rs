use bod_models::{
    schemas::config::card_plan::{
        card_plan::CardPlan, card_plan_error::CardPlanError,
        models::card_plan_with_id::CardPlanWithId,
    },
    shared::schema::BaseColleccionNames,
};
use bson::doc;
use common::utils::ntex_private::repository::public_repository::{
    PublicRepository, Repository, SetPublicRepository,
};
use futures::StreamExt;
use lazy_static::lazy_static;
use mongodb::{Client, Collection};

use std::sync::{Arc, Mutex};

use crate::modules::card_plan::models::card_plan_projection::CardPlanProjection;

lazy_static! {
    pub static ref CARD_PLAN_REPOSITORY: Arc<Mutex<Option<CardPlanRepository>>> =
        Arc::new(Mutex::new(None));
}

#[derive(Clone)]
pub struct CardPlanRepository {
    collection: Collection<CardPlan>,
    collection_id: Collection<CardPlanWithId>,
    client: Client,
}

impl Repository<CardPlan, CardPlanWithId> for CardPlanRepository {
    fn get_collection(&self) -> &Collection<CardPlan> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_collection_for_id(&self) -> &Collection<CardPlanWithId> {
        &self.collection_id
    }
}

impl CardPlanRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<CardPlan> = client
            .database(CardPlan::get_database_name())
            .collection(CardPlan::get_collection_name());
        let collection_id: Collection<CardPlanWithId> = client
            .database(CardPlan::get_database_name())
            .collection(CardPlan::get_collection_name());
        Ok(CardPlanRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }

    pub async fn get_card_plan_projection_collection(
        &self,
    ) -> Result<Vec<CardPlanProjection>, CardPlanError> {
        let mut cursor = self.collection
            .aggregate(vec![
                doc! { "$match": { "isActive": true } },
                doc! { "$sort": { "render.order": 1 } },
                doc! {
                    "$project": {
                        "button": "$render.button",
                        "price": "$render.price",
                        "shape": "$render.shape",
                        "items": "$render.items",
                        "_id": 0
                        // ... (other fields to project)
                    }
                },
            ])
            .await
            .map_err(|_| CardPlanError::GetCardPlansError("process failed getting card plan"))?;

        let mut card_plans = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let card_plan: CardPlanProjection =
                        bson::from_document(document).map_err(|_| {
                            CardPlanError::GetCardPlansError("process failed getting card plan")
                        })?;
                    card_plans.push(card_plan);
                }
                Err(_) => {
                    return Err(CardPlanError::GetCardPlansError(
                        "process failed getting card plan",
                    ))
                }
            }
        }

        Ok(card_plans)
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for CardPlanRepository {
    type RepositoryType = CardPlanRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &CARD_PLAN_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = CardPlanRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}
