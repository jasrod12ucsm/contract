use bod_models::{schemas::mst::accounting_account::{accounting_account::AccountingAccount, models::accounting_account_with_id::AccountingAccountWithId}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
        PublicRepository, Repository, SetPublicRepository,
    };
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref ACCOUNT_REPOSITORY: Arc<Mutex<Option<AccountingAccountRepository>>> =
        Arc::new(Mutex::new(None));
);

#[derive(Clone)]
pub struct AccountingAccountRepository {
    collection: Collection<AccountingAccount>,
    collection_id: Collection<AccountingAccountWithId>,
    client: Client,
}

impl Repository<AccountingAccount, AccountingAccountWithId> for AccountingAccountRepository {
    fn get_collection(&self) -> &Collection<AccountingAccount> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
    
    fn get_collection_for_id(&self) -> &Collection<AccountingAccountWithId> {
        &self.collection_id
    }
}

impl AccountingAccountRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<AccountingAccount> = client
            .database(AccountingAccount::get_database_name())
            .collection(AccountingAccount::get_collection_name());
        let collection_id: Collection<AccountingAccountWithId> = client
            .database(AccountingAccount::get_database_name())
            .collection(AccountingAccount::get_collection_name());
        Ok(AccountingAccountRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for AccountingAccountRepository {
    type RepositoryType = AccountingAccountRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &ACCOUNT_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = AccountingAccountRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}