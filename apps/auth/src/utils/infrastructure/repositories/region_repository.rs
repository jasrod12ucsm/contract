use bod_models::{schemas::location::region::{models::region_with_id::RegionWithId, region::Region}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref REGION_REPOSITORY: Arc<Mutex<Option<RegionRepository>>> =
        Arc::new(Mutex::new(None));
);
#[derive(Clone)]
pub struct RegionRepository {
    collection: Collection<Region>,
    collection_id: Collection<RegionWithId>,
    client: Client,
}

impl Repository<Region,RegionWithId> for RegionRepository {
    fn get_collection(&self) -> &Collection<Region> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<RegionWithId> {
        &self.collection_id
    }
    
}

impl RegionRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<Region> = client
            .database(Region::get_database_name())
            .collection(Region::get_collection_name());
        let collection_id: Collection<RegionWithId> = client
            .database(Region::get_database_name())
            .collection(Region::get_collection_name());
        Ok(RegionRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for RegionRepository {
    type RepositoryType = RegionRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &REGION_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = RegionRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}
