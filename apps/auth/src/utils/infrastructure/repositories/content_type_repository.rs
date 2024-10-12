use bod_models::{
    schemas::config::content_type::{content_type::ContentType, models::cnf_content_type_with_id::ContentTypeWithId}, shared::schema::BaseColleccionNames
};
use common::utils::ntex_private::repository::public_repository::{PublicRepository, Repository, SetPublicRepository};
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref CONTENT_TYPE_REPOSITORY: Arc<Mutex<Option<ContentTypeRepository>>> = Arc::new(Mutex::new(None));
}

#[derive(Clone)]
pub struct ContentTypeRepository {
    collection: Collection<ContentType>,
    collection_id: Collection<ContentTypeWithId>,
    client: Client,
}

impl Repository<ContentType, ContentTypeWithId> for ContentTypeRepository {
    fn get_collection(&self) -> &Collection<ContentType> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_collection_for_id(&self) -> &Collection<ContentTypeWithId> {
        &self.collection_id
    }
}

impl ContentTypeRepository {
    pub fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<ContentType> = client
            .database(ContentType::get_database_name())
            .collection(ContentType::get_collection_name());
        let collection_id: Collection<ContentTypeWithId> = client
            .database(ContentType::get_database_name())
            .collection(ContentType::get_collection_name());
        Ok(ContentTypeRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for ContentTypeRepository {
    type RepositoryType = ContentTypeRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &CONTENT_TYPE_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = ContentTypeRepository::new(repository)?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}