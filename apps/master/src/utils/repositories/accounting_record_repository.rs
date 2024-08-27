use bod_models::{schemas::mst::accounting_record::{accounting_record::AccountingRecord, models::accounting_record_with_id::AccountingRecordWithId}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref ACCOUNT_RECORD_REPOSITORY: Arc<Mutex<Option<AccountingRecordRepository>>> =
        Arc::new(Mutex::new(None));
);

#[derive(Clone)]
pub struct AccountingRecordRepository {
    collection: Collection<AccountingRecord>,
    collection_id: Collection<AccountingRecordWithId>,
    client: Client,
}

impl Repository<AccountingRecord, AccountingRecordWithId> for AccountingRecordRepository {
    fn get_collection(&self) -> &Collection<AccountingRecord> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<AccountingRecordWithId> {
        &self.collection_id
    }
}

impl AccountingRecordRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<AccountingRecord> = client
            .database(AccountingRecord::get_database_name())
            .collection(AccountingRecord::get_collection_name());
        let collection_id: Collection<AccountingRecordWithId> = client
            .database(AccountingRecord::get_database_name())
            .collection(AccountingRecord::get_collection_name());
        Ok(AccountingRecordRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for AccountingRecordRepository {
    type RepositoryType = AccountingRecordRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &ACCOUNT_RECORD_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = AccountingRecordRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}