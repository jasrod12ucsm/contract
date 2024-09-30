use bod_models::{schemas::mst::restaurant::{models::restaurant_with_id::RestaurantWithId, restaurant::Restaurant}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref RESTAURANT_REPOSITORY: Arc<Mutex<Option<RestaurantRepository>>> =
        Arc::new(Mutex::new(None));
);

#[derive(Clone)]
pub struct RestaurantRepository {
    collection: Collection<Restaurant>,
    collection_id: Collection<RestaurantWithId>,
    client: Client,
}

impl Repository<Restaurant, RestaurantWithId> for RestaurantRepository {
    fn get_collection(&self) -> &Collection<Restaurant> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_collection_for_id(&self) -> &Collection<RestaurantWithId> {
        &self.collection_id
    }
}

impl RestaurantRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<Restaurant> = client
            .database(Restaurant::get_database_name())
            .collection(Restaurant::get_collection_name());
        let collection_id: Collection<RestaurantWithId> = client
            .database(Restaurant::get_database_name())
            .collection(Restaurant::get_collection_name());
        Ok(RestaurantRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for RestaurantRepository {
    type RepositoryType = RestaurantRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &RESTAURANT_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = RestaurantRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}