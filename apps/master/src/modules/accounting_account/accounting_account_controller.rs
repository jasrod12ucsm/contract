use std::vec;

use bod_models::schemas::mst::accounting_account::{
    accounting_account::AccountingAccount, accounting_account_errors::AccountingAccountError,
    models::accounting_account_with_id::AccountingAccountWithId,
};
use bson::{doc, oid::ObjectId};
use common::{
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};
use futures::StreamExt;
use mongodb::results::{InsertOneResult, UpdateResult};
use ntex::web::{
    self,
    types::{Path, State},
};

use crate::utils::repositories::{
    accounting_account_repository::AccountingAccountRepository, user_repository::UserRepository,
};

#[web::post("create")]
pub async fn create_account(
    account: JsonAdvanced<AccountingAccount>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<InsertOneResult>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::CreateAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;

    let account = account.into_inner();
    //insert_one
    let account_inserted = account_repository.insert_one(account).await.map_err(|_| {
        AccountingAccountError::CreateAccountingAccountError("error inserting account")
    })?;

    Ok(JsonAdvanced(account_inserted))
}

#[web::get("{id}")]
pub async fn get_account_by_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<AccountingAccountWithId>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::GetAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;
    let account_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingAccountError::GetAccountingAccountError("cannot parse important data")
    })?;
    let account = account_repository
        .find_one(doc! {"_id":account_id}, None)
        .await
        .map_err(|_| AccountingAccountError::GetAccountingAccountError("internal data failure"))?
        .ok_or_else(|| AccountingAccountError::GetAccountingAccountError("not exist account"))?;
    Ok(JsonAdvanced(account))
}

#[web::get("user/{id}")]
pub async fn get_account_by_user_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingAccountWithId>>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::GetAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;
    let user_repository: UserRepository =
        repo.get_repository::<UserRepository>().await.map_err(|_| {
            AccountingAccountError::GetAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;
    let user_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingAccountError::GetAccountingAccountError("cannot parse important data")
    })?;
    let _user = user_repository
        .find_one(doc! {"_id":user_id}, None)
        .await
        .map_err(|_| AccountingAccountError::GetAccountingAccountError("internal data failure"))?
        .ok_or_else(|| AccountingAccountError::GetAccountingAccountError("not exist user"))?;
    let mut account = account_repository
        .find(doc! {"userId":user_id})
        .await
        .map_err(|_| AccountingAccountError::GetAccountingAccountError("internal data failure"))?;
    let mut accounts = vec![];
    while let Some(value) = account.next().await {
        if value.is_err() {
            return Err(AccountingAccountError::GetAccountingAccountError(
                "internal data failure",
            ));
        }
        let account = value.unwrap();
        accounts.push(account);
    }
    Ok(JsonAdvanced(accounts))
}

#[web::get("get_all")]
pub async fn get_all_accounts(
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingAccountWithId>>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::GetAccountingAccountsError(
                "internal error, communicate with programmers",
            )
        })?;
    let mut accounts = account_repository
        .find(doc! {})
        .await
        .map_err(|_| AccountingAccountError::GetAccountingAccountsError("internal data failure"))?;
    let mut accounts_vector = vec![];
    while let Some(account) = accounts.next().await {
        if account.is_err() {
            return Err(AccountingAccountError::GetAccountingAccountsError(
                "code error, contact with technical team",
            ));
        }
        let account = account.unwrap();
        accounts_vector.push(account);
    }
    Ok(JsonAdvanced(accounts_vector))
}

#[web::get("name/{name}")]
pub async fn get_account_by_name(
    path: Path<String>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingAccountWithId>>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::GetAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;
    let account_name = path.into_inner();
    let mut accounts = account_repository
        .find(doc! {"name": &account_name})
        .await
        .map_err(|_| AccountingAccountError::GetAccountingAccountError("internal data failure"))?;
    let mut accounts_vector = vec![];
    while let Some(account) = accounts.next().await {
        if account.is_err() {
            return Err(AccountingAccountError::GetAccountingAccountError(
                "code error, contact with technical team",
            ));
        }
        let account = account.unwrap();
        accounts_vector.push(account);
    }
    Ok(JsonAdvanced(accounts_vector))
}

#[web::put("{id}")]
pub async fn update_account(
    path: Path<IdPath>,
    account: JsonAdvanced<AccountingAccount>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<AccountingAccountWithId>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::UpdateAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;
    let account_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingAccountError::UpdateAccountingAccountError("cannot parse important data")
    })?;
    let account: AccountingAccount = account.into_inner();
    let document_to_update_account = doc! {
        "$set": bson::to_bson(&account).unwrap(),
    };
    let account_updated = account_repository
        .find_one_and_update(doc! {"_id":account_id}, document_to_update_account)
        .await
        .map_err(|_| {
            AccountingAccountError::UpdateAccountingAccountError("error updating account")
        })?
        .ok_or_else(|| {
            AccountingAccountError::UpdateAccountingAccountError("result of account is none")
        })?;
    Ok(JsonAdvanced(account_updated))
}

#[web::delete("{id}")]
pub async fn delete_account(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<UpdateResult>, AccountingAccountError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingAccountError::DeleteAccountingAccountError(
                "internal error, communicate with programmers",
            )
        })?;
    let account_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingAccountError::DeleteAccountingAccountError("cannot parse important data")
    })?;
    let filter = doc! {"_id":account_id};
    let update = doc! {"$set":doc!{"isDeleted":true}};
    let update_account_result = account_repository
        .update_one(filter, update)
        .await
        .map_err(|_| {
            AccountingAccountError::DeleteAccountingAccountError("error deleting account")
        })?;
    Ok(JsonAdvanced(update_account_result))
}