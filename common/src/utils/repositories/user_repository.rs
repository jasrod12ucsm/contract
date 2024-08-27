use crate::utils::ntex_private::repository::public_repository::{
    PublicRepository, Repository, SetPublicRepository,
};
use bod_models::{
    schemas::mst::user::{models::user_with_id::UserWithId, user::User},
    shared::schema::BaseColleccionNames,
};
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref USER_REPOSITORY: Arc<Mutex<Option<UserRepository>>> = Arc::new(Mutex::new(None));
);
#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
    collection_id: Collection<UserWithId>,
    client: Client,
}

impl Repository<User, UserWithId> for UserRepository {
    fn get_collection(&self) -> &Collection<User> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_collection_for_id(&self) -> &Collection<UserWithId> {
        &self.collection_id
    }

}

impl UserRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<User> = client
            .database(User::get_database_name())
            .collection(User::get_collection_name());
        let collection_id: Collection<UserWithId> = client
            .database(User::get_database_name())
            .collection(User::get_collection_name());
        Ok(UserRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for UserRepository {
    type RepositoryType = UserRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &USER_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = UserRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}
