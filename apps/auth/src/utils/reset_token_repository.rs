use bod_models::{schemas::config::reset_token::{models::reset_token_with_id::ResetTokenWithId, reset_token::ResetToken}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref RESET_TOKEN_REPOSITORY: Arc<Mutex<Option<ResetTokenRepository>>> =
        Arc::new(Mutex::new(None));
);
#[derive(Clone)]
pub struct ResetTokenRepository {
    collection: Collection<ResetToken>,
    collection_id: Collection<ResetTokenWithId>,
    client: Client,
}

impl Repository<ResetToken,ResetTokenWithId> for ResetTokenRepository {
    fn get_collection(&self) -> &Collection<ResetToken> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<ResetTokenWithId> {
        &self.collection_id
    }
}

impl ResetTokenRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<ResetToken> = client
            .database(ResetToken::get_database_name())
            .collection(ResetToken::get_collection_name());
        let collection_id: Collection<ResetTokenWithId> = client
            .database(ResetToken::get_database_name())
            .collection(ResetToken::get_collection_name());
        Ok(ResetTokenRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for ResetTokenRepository {
    type RepositoryType = ResetTokenRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &RESET_TOKEN_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = ResetTokenRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}
