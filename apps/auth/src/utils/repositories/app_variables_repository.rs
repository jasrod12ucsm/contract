use bod_models::{schemas::app::variables::{models::variables_with_id::AppVariablesWithId, variables::AppVariables}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref APP_VARIABLES_REPOSITORY: Arc<Mutex<Option<AppVariablesRepository>>> =
        Arc::new(Mutex::new(None));
);

#[derive(Clone)]
pub struct AppVariablesRepository {
    collection: Collection<AppVariables>,
    collection_id: Collection<AppVariablesWithId>,
    client: Client,
}

impl Repository<AppVariables, AppVariablesWithId> for AppVariablesRepository {
    fn get_collection(&self) -> &Collection<AppVariables> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<AppVariablesWithId> {
        &self.collection_id
    }
}

impl AppVariablesRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<AppVariables> = client
            .database(AppVariables::get_database_name())
            .collection(AppVariables::get_collection_name());
        let collection_id: Collection<AppVariablesWithId> = client
            .database(AppVariables::get_database_name())
            .collection(AppVariables::get_collection_name());
        Ok(AppVariablesRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for AppVariablesRepository {
    type RepositoryType = AppVariablesRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &APP_VARIABLES_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = AppVariablesRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}