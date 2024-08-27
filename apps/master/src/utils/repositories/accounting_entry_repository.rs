use bod_models::{schemas::mst::accounting_entry::{accounting_entry::AccountingEntry, models::accounting_entry_with_id::AccountingEntryWithId}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref ACCOUNT_ENTRY_REPOSITORY: Arc<Mutex<Option<AccountingEntryRepository>>> =
        Arc::new(Mutex::new(None));
);

#[derive(Clone)]
pub struct AccountingEntryRepository {
    collection: Collection<AccountingEntry>,
    collection_id: Collection<AccountingEntryWithId>,
    client: Client,
}

impl Repository<AccountingEntry, AccountingEntryWithId> for AccountingEntryRepository {
    fn get_collection(&self) -> &Collection<AccountingEntry> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<AccountingEntryWithId> {
        &self.collection_id
    }
}

impl AccountingEntryRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<AccountingEntry> = client
            .database(AccountingEntry::get_database_name())
            .collection(AccountingEntry::get_collection_name());
        let collection_id: Collection<AccountingEntryWithId> = client
            .database(AccountingEntry::get_database_name())
            .collection(AccountingEntry::get_collection_name());
        Ok(AccountingEntryRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for AccountingEntryRepository {
    type RepositoryType = AccountingEntryRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &ACCOUNT_ENTRY_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = AccountingEntryRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}