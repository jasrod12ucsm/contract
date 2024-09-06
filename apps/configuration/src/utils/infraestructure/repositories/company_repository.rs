use bod_models::{
    schemas::config::company::{company::Company, models::company_with_id::CompanyWithId},
    shared::schema::BaseColleccionNames,
};
use common::utils::ntex_private::repository::public_repository::{
    PublicRepository, Repository, SetPublicRepository,
};
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref COMPANY_REPOSITORY: Arc<Mutex<Option<CompanyRepository>>> =
        Arc::new(Mutex::new(None));
}

#[derive(Clone)]
pub struct CompanyRepository {
    collection: Collection<Company>,
    collection_id: Collection<CompanyWithId>,
    client: Client,
}

impl Repository<Company, CompanyWithId> for CompanyRepository {
    fn get_collection(&self) -> &Collection<Company> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_collection_for_id(&self) -> &Collection<CompanyWithId> {
        &self.collection_id
    }
}

impl CompanyRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<Company> = client
            .database(Company::get_database_name())
            .collection(Company::get_collection_name());
        let collection_id: Collection<CompanyWithId> = client
            .database(Company::get_database_name())
            .collection(Company::get_collection_name());
        Ok(CompanyRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for CompanyRepository {
    type RepositoryType = CompanyRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &COMPANY_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = CompanyRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}