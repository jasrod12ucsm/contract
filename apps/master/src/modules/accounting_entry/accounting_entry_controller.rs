use bod_models::schemas::mst::accounting_entry::{
    accounting_entry::AccountingEntry, accounting_entry_errors::AccountingEntryError,
    models::accounting_entry_with_id::AccountingEntryWithId,
};
use bson::{doc, oid::ObjectId};
use chrono::{Datelike, TimeZone};
use common::{
    public::models::path::{IdPath, IdPathThreeRangeDates, IdPathTwo, IdPathTwoMonth},
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

use crate::{
    modules::accounting_entry::models::account_entry_view::AccountingEntryView,
    utils::repositories::{
        accounting_account_repository::AccountingAccountRepository,
        accounting_entry_repository::AccountingEntryRepository,
    },
};

#[web::post("create")]
pub async fn create_entry(
    entry: JsonAdvanced<AccountingEntry>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<InsertOneResult>, AccountingEntryError> {
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::CreateAccountingEntryError(
                "internal error, communicate with programmers",
            )
        })?;

    let entry = entry.into_inner();
    let entry_inserted = entry_repository
        .insert_one(entry)
        .await
        .map_err(|_| AccountingEntryError::CreateAccountingEntryError("error inserting entry"))?;

    Ok(JsonAdvanced(entry_inserted))
}

#[web::get("{id}")]
pub async fn get_entry_by_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<AccountingEntryWithId>, AccountingEntryError> {
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntryError(
                "internal error, communicate with programmers",
            )
        })?;
    let entry_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingEntryError::GetAccountingEntryError("cannot parse important data")
    })?;
    let entry = entry_repository
        .find_one(doc! {"_id":entry_id}, None)
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntryError("internal data failure"))?
        .ok_or_else(|| AccountingEntryError::GetAccountingEntryError("not exist entry"))?;
    Ok(JsonAdvanced(entry))
}

#[web::get("get_all")]
pub async fn get_all_entries(
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingEntryWithId>>, AccountingEntryError> {
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;
    let mut entries = entry_repository
        .find(doc! {})
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;
    let mut entries_vector = vec![];
    while let Some(entry) = entries.next().await {
        if entry.is_err() {
            return Err(AccountingEntryError::GetAccountingEntriesError(
                "code error, contact with technical team",
            ));
        }
        let entry = entry.unwrap();
        entries_vector.push(entry);
    }
    Ok(JsonAdvanced(entries_vector))
}

#[web::put("{id}")]
pub async fn update_entry(
    path: Path<IdPath>,
    entry: JsonAdvanced<AccountingEntry>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<AccountingEntryWithId>, AccountingEntryError> {
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::UpdateAccountingEntryError(
                "internal error, communicate with programmers",
            )
        })?;
    let entry_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingEntryError::UpdateAccountingEntryError("cannot parse important data")
    })?;
    let entry: AccountingEntry = entry.into_inner();
    let document_to_update_entry = doc! {
        "$set": bson::to_bson(&entry).unwrap(),
    };
    let entry_updated = entry_repository
        .find_one_and_update(doc! {"_id":entry_id}, document_to_update_entry)
        .await
        .map_err(|_| AccountingEntryError::UpdateAccountingEntryError("error updating entry"))?
        .ok_or_else(|| {
            AccountingEntryError::UpdateAccountingEntryError("result of entry is none")
        })?;
    Ok(JsonAdvanced(entry_updated))
}

#[web::delete("{id}")]
pub async fn delete_entry(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<UpdateResult>, AccountingEntryError> {
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::DeleteAccountingEntryError(
                "internal error, communicate with programmers",
            )
        })?;
    let entry_id = ObjectId::parse_str(path.id()).map_err(|_| {
        AccountingEntryError::DeleteAccountingEntryError("cannot parse important data")
    })?;
    let filter = doc! {"_id":entry_id};
    let update = doc! {"$set":doc!{"isDeleted":true}};
    let update_entry_result = entry_repository
        .update_one(filter, update)
        .await
        .map_err(|_| AccountingEntryError::DeleteAccountingEntryError("error deleting entry"))?;
    Ok(JsonAdvanced(update_entry_result))
}
#[web::get("account_entries/{id}/{id_2}")]
pub async fn get_account_entries(
    path: Path<IdPathTwo>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingEntryView>>, AccountingEntryError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;

    let account_id = ObjectId::parse_str(&path.id()).map_err(|_| {
        AccountingEntryError::GetAccountingEntriesError("cannot parse important data")
    })?;

    let company_id = ObjectId::parse_str(&path.id_2()).map_err(|_| {
        AccountingEntryError::GetAccountingEntriesError("cannot parse company data")
    })?;

    // Obtener la cuenta principal
    let main_account = account_repository
        .find_one(doc! {"_id": account_id, "companyId": company_id}, None)
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?
        .ok_or_else(|| AccountingEntryError::GetAccountingEntriesError("account not found"))?;

    // Obtener las subcuentas
    let mut sub_accounts = account_repository
        .find(doc! {"parentAccount": account_id, "companyId": company_id})
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;

    let mut account_ids = vec![account_id];
    while let Some(sub_account) = sub_accounts.next().await {
        if let Ok(sub_account) = sub_account {
            account_ids.push(sub_account.id);
        }
    }

    // Obtener todas las entradas contables asociadas a la cuenta principal y sus subcuentas
    let mut entries = entry_repository
        .find(doc! {"accountingAccountId": {"$in": account_ids}, "companyId": company_id})
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;

    let mut entries_vector = vec![];
    while let Some(entry) = entries.next().await {
        if let Ok(entry) = entry {
            entries_vector.push(entry);
        }
    }

    // Construir la vista jerárquica
    let mut hierarchical_entries = vec![];
    for entry in entries_vector {
        let account_number = if entry.accounting_account_id == account_id {
            main_account.account_number.clone()
        } else {
            format!(
                "{}.{}",
                main_account.account_number, entry.accounting_account_id
            )
        };
        hierarchical_entries.push(AccountingEntryView {
            account_number: account_number,
            description: entry.description.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
            is_active: entry.is_active,
            is_deleted: entry.is_deleted,
            credit: entry.credit,
            debit: entry.debit,
        });
    }

    Ok(JsonAdvanced(hierarchical_entries))
}

#[web::get("account_entries/{id}/{id_2}/{month}")]
pub async fn get_account_entries_by_month(
    path: Path<IdPathTwoMonth>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingEntryView>>, AccountingEntryError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;

    let account_id = ObjectId::parse_str(&path.id()).map_err(|_| {
        AccountingEntryError::GetAccountingEntriesError("cannot parse important data")
    })?;

    let company_id = ObjectId::parse_str(&path.id2()).map_err(|_| {
        AccountingEntryError::GetAccountingEntriesError("cannot parse company data")
    })?;

    let naive_date = if let Ok(date) = chrono::NaiveDate::parse_from_str(&path.month(), "%Y-%m-%d")
    {
        date
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(&path.month(), "%Y-%m") {
        date.and_hms_opt(0, 0, 0)
            .ok_or_else(|| {
                AccountingEntryError::GetAccountingEntriesError("cannot set start time")
            })?
            .date()
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(&path.month(), "%Y") {
        date.and_hms_opt(0, 0, 0)
            .ok_or_else(|| {
                AccountingEntryError::GetAccountingEntriesError("cannot set start time")
            })?
            .date()
    } else {
        return Err(AccountingEntryError::GetAccountingEntriesError(
            "cannot parse date",
        ));
    };

    let (start_date, end_date) = if path.month().len() == 10 {
        // Year-Month-Day format
        let start_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?)
                .timestamp_millis(),
        );
        let end_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&naive_date.and_hms_opt(23, 59, 59).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set end time")
                })?)
                .timestamp_millis(),
        );
        (start_date, end_date)
    } else if path.month().len() == 7 {
        // Year-Month format
        let start_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?)
                .timestamp_millis(),
        );
        let end_date = if naive_date.month() == 12 {
            bson::DateTime::from_millis(
                chrono::Utc
                    .from_utc_datetime(
                        &naive_date
                            .with_year(naive_date.year() + 1)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError("cannot set year")
                            })?
                            .with_month(1)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError("cannot set month")
                            })?
                            .and_hms_opt(0, 0, 0)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError(
                                    "cannot set end time",
                                )
                            })?,
                    )
                    .timestamp_millis(),
            )
        } else {
            bson::DateTime::from_millis(
                chrono::Utc
                    .from_utc_datetime(
                        &naive_date
                            .with_month(naive_date.month() + 1)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError("cannot set month")
                            })?
                            .and_hms_opt(0, 0, 0)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError(
                                    "cannot set end time",
                                )
                            })?,
                    )
                    .timestamp_millis(),
            )
        };
        (start_date, end_date)
    } else {
        // Year format
        let start_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?)
                .timestamp_millis(),
        );
        let end_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(
                    &naive_date
                        .with_year(naive_date.year() + 1)
                        .ok_or_else(|| {
                            AccountingEntryError::GetAccountingEntriesError("cannot set year")
                        })?
                        .and_hms_opt(0, 0, 0)
                        .ok_or_else(|| {
                            AccountingEntryError::GetAccountingEntriesError("cannot set end time")
                        })?,
                )
                .timestamp_millis(),
        );
        (start_date, end_date)
    };

    // Fetch the main account
    let main_account = account_repository
        .find_one(doc! {"_id": account_id, "companyId": company_id}, None)
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?
        .ok_or_else(|| AccountingEntryError::GetAccountingEntriesError("account not found"))?;

    // Fetch sub-accounts
    let mut sub_accounts = account_repository
        .find(doc! {"parentAccount": account_id, "companyId": company_id})
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;

    let mut account_ids = vec![account_id];
    while let Some(sub_account) = sub_accounts.next().await {
        if let Ok(sub_account) = sub_account {
            account_ids.push(sub_account.id);
        }
    }

    // Fetch all accounting entries associated with the main account and its sub-accounts for the specified month
    let mut entries = entry_repository
        .find(doc! {
            "accountingAccountId": {"$in": account_ids},
            "companyId": company_id,
            "createdAt": {
                "$gte": start_date,
                "$lt": end_date
            }
        })
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;

    let mut entries_vector = vec![];
    while let Some(entry) = entries.next().await {
        if let Ok(entry) = entry {
            entries_vector.push(entry);
        }
    }

    // Build the hierarchical view
    let mut hierarchical_entries = vec![];
    for entry in entries_vector {
        let account_number = if entry.accounting_account_id == account_id {
            main_account.account_number.clone()
        } else {
            format!(
                "{}.{}",
                main_account.account_number, entry.accounting_account_id
            )
        };
        hierarchical_entries.push(AccountingEntryView {
            account_number: account_number,
            description: entry.description.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
            is_active: entry.is_active,
            is_deleted: entry.is_deleted,
            credit: entry.credit,
            debit: entry.debit,
        });
    }

    Ok(JsonAdvanced(hierarchical_entries))
}


#[web::get("account_entries/{id}/{id_2}/{start_date}/{end_date}")]
pub async fn get_account_entries_by_date_range(
    path: Path<IdPathThreeRangeDates>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<AccountingEntryView>>, AccountingEntryError> {
    let account_repository: AccountingAccountRepository = repo
        .get_repository::<AccountingAccountRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;
    let entry_repository: AccountingEntryRepository = repo
        .get_repository::<AccountingEntryRepository>()
        .await
        .map_err(|_| {
            AccountingEntryError::GetAccountingEntriesError(
                "internal error, communicate with programmers",
            )
        })?;

    let account_id = ObjectId::parse_str(&path.id()).map_err(|_| {
        AccountingEntryError::GetAccountingEntriesError("cannot parse important data")
    })?;

    let company_id = ObjectId::parse_str(&path.id2()).map_err(|_| {
        AccountingEntryError::GetAccountingEntriesError("cannot parse company data")
    })?;

    let parse_date = |date_str: &str| -> Result<chrono::NaiveDate, AccountingEntryError> {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date)
        } else if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m") {
            Ok(date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?
                .date())
        } else if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y") {
            Ok(date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?
                .date())
        } else {
            Err(AccountingEntryError::GetAccountingEntriesError(
                "cannot parse date",
            ))
        }
    };

    let start_naive_date = parse_date(&path.start_date())?;
    let end_naive_date = parse_date(&path.end_date())?;

    let (start_date, end_date) = if path.start_date().len() == 10 && path.end_date().len() == 10 {
        // Year-Month-Day format
        let start_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&start_naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?)
                .timestamp_millis(),
        );
        let end_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&end_naive_date.and_hms_opt(23, 59, 59).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set end time")
                })?)
                .timestamp_millis(),
        );
        (start_date, end_date)
    } else if path.start_date().len() == 7 && path.end_date().len() == 7 {
        // Year-Month format
        let start_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&start_naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?)
                .timestamp_millis(),
        );
        let end_date = if end_naive_date.month() == 12 {
            bson::DateTime::from_millis(
                chrono::Utc
                    .from_utc_datetime(
                        &end_naive_date
                            .with_year(end_naive_date.year() + 1)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError("cannot set year")
                            })?
                            .with_month(1)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError("cannot set month")
                            })?
                            .and_hms_opt(0, 0, 0)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError(
                                    "cannot set end time",
                                )
                            })?,
                    )
                    .timestamp_millis(),
            )
        } else {
            bson::DateTime::from_millis(
                chrono::Utc
                    .from_utc_datetime(
                        &end_naive_date
                            .with_month(end_naive_date.month() + 1)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError("cannot set month")
                            })?
                            .and_hms_opt(0, 0, 0)
                            .ok_or_else(|| {
                                AccountingEntryError::GetAccountingEntriesError(
                                    "cannot set end time",
                                )
                            })?,
                    )
                    .timestamp_millis(),
            )
        };
        (start_date, end_date)
    } else {
        // Year format
        let start_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(&start_naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                    AccountingEntryError::GetAccountingEntriesError("cannot set start time")
                })?)
                .timestamp_millis(),
        );
        let end_date = bson::DateTime::from_millis(
            chrono::Utc
                .from_utc_datetime(
                    &end_naive_date
                        .with_year(end_naive_date.year() + 1)
                        .ok_or_else(|| {
                            AccountingEntryError::GetAccountingEntriesError("cannot set year")
                        })?
                        .and_hms_opt(0, 0, 0)
                        .ok_or_else(|| {
                            AccountingEntryError::GetAccountingEntriesError("cannot set end time")
                        })?,
                )
                .timestamp_millis(),
        );
        (start_date, end_date)
    };

    // Fetch the main account
    let main_account = account_repository
        .find_one(doc! {"_id": account_id, "companyId": company_id}, None)
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?
        .ok_or_else(|| AccountingEntryError::GetAccountingEntriesError("account not found"))?;

    // Fetch sub-accounts
    let mut sub_accounts = account_repository
        .find(doc! {"parentAccount": account_id, "companyId": company_id})
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;

    let mut account_ids = vec![account_id];
    while let Some(sub_account) = sub_accounts.next().await {
        if let Ok(sub_account) = sub_account {
            account_ids.push(sub_account.id);
        }
    }

    // Fetch all accounting entries associated with the main account and its sub-accounts for the specified date range
    let mut entries = entry_repository
        .find(doc! {
            "accountingAccountId": {"$in": account_ids},
            "companyId": company_id,
            "createdAt": {
                "$gte": start_date,
                "$lt": end_date
            }
        })
        .await
        .map_err(|_| AccountingEntryError::GetAccountingEntriesError("internal data failure"))?;

    let mut entries_vector = vec![];
    while let Some(entry) = entries.next().await {
        if let Ok(entry) = entry {
            entries_vector.push(entry);
        }
    }

    // Build the hierarchical view
    let mut hierarchical_entries = vec![];
    for entry in entries_vector {
        let account_number = if entry.accounting_account_id == account_id {
            main_account.account_number.clone()
        } else {
            format!(
                "{}.{}",
                main_account.account_number, entry.accounting_account_id
            )
        };
        hierarchical_entries.push(AccountingEntryView {
            account_number: account_number,
            description: entry.description.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
            is_active: entry.is_active,
            is_deleted: entry.is_deleted,
            credit: entry.credit,
            debit: entry.debit,
        });
    }

    Ok(JsonAdvanced(hierarchical_entries))
}
