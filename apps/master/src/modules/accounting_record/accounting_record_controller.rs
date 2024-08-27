use bod_models::schemas::mst::accounting_record::{
    accounting_record::AccountingRecord, accounting_record_errors::AccountingRecordError,
    models::accounting_record_with_id::AccountingRecordWithId,
};
use bson::{doc, oid::ObjectId, DateTime, Document};
use mongodb::results::{InsertOneResult, UpdateResult};

use common::{
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};
use futures::StreamExt;
use ntex::web::{
    self,
    types::{Path, State},
};

use crate::{
    modules::accounting_record::data::{
        create_accounting_record_request::CreateAccountingRecordRequest,
        update_accounting_record_request::UpdateAccountingRecordRequest,
    },
    utils::repositories::accounting_record_repository::AccountingRecordRepository,
};

#[web::post("create")]
pub async fn create_record(
    record: JsonAdvanced<CreateAccountingRecordRequest>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<InsertOneResult>, AccountingRecordError> {
    let record_repository: AccountingRecordRepository = repo
        .get_repository::<AccountingRecordRepository>()
        .await
        .map_err(|_| {
            AccountingRecordError::CreateAccountingRecordError(
                "internal error, communicate with programmers",
            )
        })?;

    let CreateAccountingRecordRequest {
        accounting_entries,
        company,
        transaction_document,
    } = record.into_inner();
    let record = AccountingRecord::new(accounting_entries, transaction_document, company);

    let record_inserted = record_repository.insert_one(record).await.map_err(|_| {
        AccountingRecordError::CreateAccountingRecordError("error inserting record")
    })?;

    Ok(JsonAdvanced(record_inserted))
}

#[web::get("{id}")]
pub async fn get_record_by_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<AccountingRecordWithId>, AccountingRecordError> {
    let record_repository: AccountingRecordRepository = repo
        .get_repository::<AccountingRecordRepository>()
        .await
        .map_err(|_| {
            AccountingRecordError::GetAccountingRecordError(
                "internal error, communicate with programmers",
            )
        })?;
    let record_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingRecordError::GetAccountingRecordError("cannot parse important data")
    })?;
    let record = record_repository
        .find_one(doc! {"_id":record_id}, None)
        .await
        .map_err(|_| AccountingRecordError::GetAccountingRecordError("internal data failure"))?
        .ok_or_else(|| AccountingRecordError::AccountingRecordNotFoundError("not exist record"))?;
    Ok(JsonAdvanced(record))
}

#[web::get("get_all")]
pub async fn get_all_records(
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingRecordWithId>>, AccountingRecordError> {
    let record_repository: AccountingRecordRepository = repo
        .get_repository::<AccountingRecordRepository>()
        .await
        .map_err(|_| {
            AccountingRecordError::GetAccountingRecordsError(
                "internal error, communicate with programmers",
            )
        })?;
    let mut records = record_repository
        .find(doc! {})
        .await
        .map_err(|_| AccountingRecordError::GetAccountingRecordsError("internal data failure"))?;
    let mut records_vector = vec![];
    while let Some(record) = records.next().await {
        if record.is_err() {
            return Err(AccountingRecordError::GetAccountingRecordsError(
                "code error, contact with technical team",
            ));
        }
        let record = record.unwrap();
        records_vector.push(record);
    }
    Ok(JsonAdvanced(records_vector))
}

#[web::put("{id}")]
pub async fn update_record(
    path: Path<IdPath>,
    record: JsonAdvanced<UpdateAccountingRecordRequest>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<AccountingRecordWithId>, AccountingRecordError> {
    let record_repository: AccountingRecordRepository = repo
        .get_repository::<AccountingRecordRepository>()
        .await
        .map_err(|_| {
            AccountingRecordError::UpdateAccountingRecordError(
                "internal error, communicate with programmers",
            )
        })?;
    let record_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingRecordError::UpdateAccountingRecordError("cannot parse important data")
    })?;
    let record_request: UpdateAccountingRecordRequest = record.into_inner();

    let mut document_to_update_record = Document::new();
    if let Some(accounting_entries) = record_request.accounting_entries {
        document_to_update_record.insert(
            "accountingEntries",
            bson::to_bson(&accounting_entries).unwrap(),
        );
    }
    if let Some(transaction_document) = record_request.transaction_document {
        document_to_update_record.insert(
            "transactionDocument",
            bson::to_bson(&transaction_document).unwrap(),
        );
    }
    if let Some(company) = record_request.company {
        document_to_update_record.insert("company", bson::to_bson(&company).unwrap());
    }
    document_to_update_record.insert("updatedAt", DateTime::now());
    let record_updated = record_repository
        .find_one_and_update(doc! {"_id":record_id}, document_to_update_record)
        .await
        .map_err(|_| AccountingRecordError::UpdateAccountingRecordError("error updating record"))?
        .ok_or_else(|| {
            AccountingRecordError::UpdateAccountingRecordError("result of record is none")
        })?;
    Ok(JsonAdvanced(record_updated))
}

#[web::delete("{id}")]
pub async fn delete_record(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<UpdateResult>, AccountingRecordError> {
    let record_repository: AccountingRecordRepository = repo
        .get_repository::<AccountingRecordRepository>()
        .await
        .map_err(|_| {
            AccountingRecordError::DeleteAccountingRecordError(
                "internal error, communicate with programmers",
            )
        })?;
    let record_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingRecordError::DeleteAccountingRecordError("cannot parse important data")
    })?;
    let filter = doc! {"_id":record_id};
    let update = doc! {"$set":doc!{"isDeleted":true}};
    let update_record_result = record_repository
        .update_one(filter, update)
        .await
        .map_err(|_| AccountingRecordError::DeleteAccountingRecordError("error deleting record"))?;
    Ok(JsonAdvanced(update_record_result))
}
