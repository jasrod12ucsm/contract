use bod_models::{schemas::config::user_config::{models::user_config_with_id::UserConfigWithId, user_config::UserConfig}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref USER_CONFIG_REPOSITORY: Arc<Mutex<Option<UserConfigRepository>>> =
        Arc::new(Mutex::new(None));
);
#[derive(Clone)]
pub struct UserConfigRepository {
    collection: Collection<UserConfig>,
    collection_id: Collection<UserConfigWithId>,
    client: Client,
}

impl Repository<UserConfig,UserConfigWithId> for UserConfigRepository {
    fn get_collection(&self) -> &Collection<UserConfig> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<UserConfigWithId> {
        &self.collection_id
    }
}

impl UserConfigRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<UserConfig> = client
            .database(UserConfig::get_database_name())
            .collection(UserConfig::get_collection_name());
        let collection_id: Collection<UserConfigWithId> = client
            .database(UserConfig::get_database_name())
            .collection(UserConfig::get_collection_name());
        Ok(UserConfigRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for UserConfigRepository {
    type RepositoryType = UserConfigRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &USER_CONFIG_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = UserConfigRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
   
}
