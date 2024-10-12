use bod_models::{schemas::location::country::{country::Country, models::country_with_id::CountryWithId}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref COUNTRY_REPOSITORY: Arc<Mutex<Option<CountryRepository>>> =
        Arc::new(Mutex::new(None));
);
#[derive(Clone)]
pub struct CountryRepository {
    collection: Collection<Country>,
    collection_id: Collection<CountryWithId>,
    client: Client,
}

impl Repository<Country,CountryWithId> for CountryRepository {
    fn get_collection(&self) -> &Collection<Country> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<CountryWithId> {
        &self.collection_id
    }
}

impl CountryRepository {
    pub fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<Country> = client
            .database(Country::get_database_name())
            .collection(Country::get_collection_name());
        let collection_id: Collection<CountryWithId> = client
            .database(Country::get_database_name())
            .collection(Country::get_collection_name());
        Ok(CountryRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for CountryRepository {
    type RepositoryType = CountryRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &COUNTRY_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = CountryRepository::new(repository)?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}
