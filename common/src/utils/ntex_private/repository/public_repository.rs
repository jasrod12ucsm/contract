use std::time::Duration;

use async_trait::async_trait;
use bod_models::shared::schema::BaseColleccionNames;
use mongodb::{
    action::{FindOne, FindOneAndUpdate, InsertOne, Update},
    bson::{doc, DateTime},
    options::{
        ClientOptions, Compressor, ReadPreference, ReadPreferenceOptions, ReturnDocument,
        SelectionCriteria, ServerApiVersion,
    },
    Client, Collection,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{helpers::env::env::ENV, utils::database::{domain::{filter_query::FilterQueryTrait, update_query::UpdateQueryTrait}, infrastructure::database_library::{FindQuery, UpdateQuery}}};
#[derive(Debug, Clone)]
pub struct PublicRepository {
    client: Option<Client>,
}

impl PublicRepository {
    pub async fn connect() -> Result<Self, mongodb::error::Error> {
        let database_string = ENV
            .get_string("DATABASE_URL")
            .expect("DATABASE_URL is missing");
        let mut client_options = ClientOptions::parse(database_string).await?;
        // Establece la opción allowUseDisk en true
        let read_preference = ReadPreference::Secondary {
            options: Some(ReadPreferenceOptions::builder().build()),
        };
        client_options.selection_criteria =
            Some(SelectionCriteria::ReadPreference(read_preference));
        client_options.default_database = Some("bod".to_string());
        client_options.app_name = Some("MyApp".to_string());
        client_options.server_api = Some(
            mongodb::options::ServerApi::builder()
                .version(ServerApiVersion::V1)
                .strict(true)
                .deprecation_errors(true)
                .build(),
        );
        client_options.retry_reads = Some(true);
        client_options.retry_writes = Some(true);
        client_options.max_pool_size = Some(8);
        client_options.read_concern = Some(mongodb::options::ReadConcern::local());
        client_options.write_concern = Some(mongodb::options::WriteConcern::majority());
        client_options.compressors = Some(vec![Compressor::Snappy]);
        client_options.local_threshold = Some(Duration::from_millis(15));
        client_options.heartbeat_freq = Some(Duration::from_secs(10));
        let client = Client::with_options(client_options)?;
        Ok(Self {
            client: Some(client),
        })
    }

    pub fn get_client(&self) -> Result<&Client, mongodb::error::Error> {
        self.client
            .as_ref()
            .ok_or_else(|| mongodb::error::Error::custom("Client not found"))
    }

    pub async fn get_repository<'a, T: SetPublicRepository>(
        &self,
    ) -> Result<<T as SetPublicRepository>::RepositoryType, mongodb::error::Error> {
        return T::set_repository(self).await;
    }
}

#[async_trait]
pub trait SetPublicRepository {
    type RepositoryType: Send;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error>;
}

pub trait AbstractRepository<T: Serialize + Send + Sync, U: Serialize + Send + Sync> {
    fn construct_new_collection<K>(&self) -> Collection<K>
    where
        K: Serialize + DeserializeOwned + Unpin + Send + Sync;
    fn insert_one(&self, item: T) -> InsertOne;
    fn find_one(
        &self,
        filter: mongodb::bson::Document,
    ) -> FindOne<U>;
    fn find_one_and_update(
        &self,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
    ) -> FindOneAndUpdate<U>; // Cambiado a Result<Option<U>, Error>
    fn update_one(
        &self,
        update_query:UpdateQuery
    ) -> Update;
    fn get_all(&self) -> mongodb::action::Find<'_, U>;
    fn find(&self, filter: FindQuery) -> mongodb::action::Find<'_, U>;
    fn find_generic<'a, J>(
        &'a self,
        filter: mongodb::bson::Document,
        collection: &'a Collection<J>,
    ) -> mongodb::action::Find<'_, J>
    where
        J: Serialize + DeserializeOwned + Unpin + Send + Sync;
}

#[async_trait]
impl<T, U, R> AbstractRepository<T, U> for R
where
    T: Serialize + Send + Sync + 'static + Unpin + DeserializeOwned + BaseColleccionNames,
    R: Repository<T, U> + Send + Sync,
    U: Serialize + Send + Sync + 'static + Unpin + DeserializeOwned,
{
    fn insert_one(&self, item: T) -> InsertOne {
        let collection = self.get_collection();
        collection.insert_one(item)
    }

    fn find_one(
        &self,
        mut filter: mongodb::bson::Document,
    ) -> FindOne<U> {
        let has_is_deleted = filter.contains_key("isDeleted");
        let has_is_active = filter.contains_key("isActive");
        let has_no_deleted = filter.contains_key("noDeleted");
        let has_no_active = filter.contains_key("noActive");

        // If neither "isDeleted" nor "isActive" are present, add default values
        if !has_is_deleted && !has_is_active && !has_no_deleted && !has_no_active {
            filter.insert("isDeleted", false);
            filter.insert("isActive", true);
        } else {
            // If "isDeleted" is not present and "noDeleted" is not present, add the default value
            if !has_is_deleted && !has_no_deleted {
                filter.insert("isDeleted", false);
            }
            // If "isActive" is not present and "noActive" is not present, add the default value
            if !has_is_active && !has_no_active {
                filter.insert("isActive", true);
            }
        }

        // Remove "noDeleted" and "noActive" from the filter if they exist
        if has_no_deleted {
            filter.remove("noDeleted");
        }
        if has_no_active {
            filter.remove("noActive");
        }
        println!("Filter: {:?}", filter);
        let collection = self.get_collection_for_id();
        collection.find_one(filter).selection_criteria(SelectionCriteria::ReadPreference(ReadPreference::PrimaryPreferred { options: None }))
    }

    fn find_one_and_update(
        &self,
        mut filter: mongodb::bson::Document,
        mut update: mongodb::bson::Document,
    ) -> FindOneAndUpdate<U> {
        let has_is_deleted = filter.contains_key("isDeleted");
        let has_is_active = filter.contains_key("isActive");
        let has_no_deleted = filter.contains_key("noDeleted");
        let has_no_active = filter.contains_key("noActive");

        // If neither "isDeleted" nor "isActive" are present, add default values
        if !has_is_deleted && !has_is_active && !has_no_deleted && !has_no_active {
            filter.insert("isDeleted", false);
            filter.insert("isActive", true);
        } else {
            // If "isDeleted" is not present and "noDeleted" is not present, add the default value
            if !has_is_deleted && !has_no_deleted {
                filter.insert("isDeleted", false);
            }
            // If "isActive" is not present and "noActive" is not present, add the default value
            if !has_is_active && !has_no_active {
                filter.insert("isActive", true);
            }
        }

        // Remove "noDeleted" and "noActive" from the filter if they exist
        if has_no_deleted {
            filter.remove("noDeleted");
        }
        if has_no_active {
            filter.remove("noActive");
        }

        let collection = self.get_collection_for_id();
        let now = DateTime::now();
        if let Ok(set_doc) = update.get_document_mut("$set") {
            set_doc.insert("updatedAt", now);
        } else {
            update.insert("$set", doc! {"updatedAt": now});
        }

        update.insert("$setOnInsert", doc! {"createdAt": now});
        println!("filter: {}", filter);
        println!("update: {}", update);
        let document = collection
            .find_one_and_update(filter, update)
            .return_document(ReturnDocument::After);
        document
    }

    fn update_one(
        &self,
        update_query: UpdateQuery,
    ) -> Update {
        let mut filter =update_query.create_filter_doc();
        let mut update=update_query.create_update_doc();
        let has_is_deleted = filter.contains_key("isDeleted");
        let has_is_active = filter.contains_key("isActive");
        let has_no_deleted = filter.contains_key("noDeleted");
        let has_no_active = filter.contains_key("noActive");

        // If neither "isDeleted" nor "isActive" are present, add default values
        if !has_is_deleted && !has_is_active && !has_no_deleted && !has_no_active {
            filter.insert("isDeleted", false);
            filter.insert("isActive", true);
        } else {
            // If "isDeleted" is not present and "noDeleted" is not present, add the default value
            if !has_is_deleted && !has_no_deleted {
                filter.insert("isDeleted", false);
            }
            // If "isActive" is not present and "noActive" is not present, add the default value
            if !has_is_active && !has_no_active {
                filter.insert("isActive", true);
            }
        }

        // Remove "noDeleted" and "noActive" from the filter if they exist
        if has_no_deleted {
            filter.remove("noDeleted");
        }
        if has_no_active {
            filter.remove("noActive");
        }

        let collection = self.get_collection_for_id();
        let now = DateTime::now();
        if let Ok(set_doc) = update.get_document_mut("$set") {
            set_doc.insert("updatedAt", now);
        } else {
            update.insert("$set", doc! {"updatedAt": now});
        }
        let document = collection.update_one(filter, update);
        document
    }

    fn get_all(&self) -> mongodb::action::Find<'_, U> {
        let collection = self.get_collection_for_id();
        let document = collection.find(doc! {});
        document
    }

    fn find(&self,filter: FindQuery) -> mongodb::action::Find<'_, U> {
        let mut filter=filter.create_filter_doc();
        // Check if "isDeleted" or "isActive" are present in the filter
        let has_is_deleted = filter.contains_key("isDeleted");
        let has_is_active = filter.contains_key("isActive");
        let has_no_deleted = filter.contains_key("noDeleted");
        let has_no_active = filter.contains_key("noActive");

        // If neither "isDeleted" nor "isActive" are present, add default values
        if !has_is_deleted && !has_is_active && !has_no_deleted && !has_no_active {
            filter.insert("isDeleted", false);
            filter.insert("isActive", true);
        } else {
            // If "isDeleted" is not present and "noDeleted" is not present, add the default value
            if !has_is_deleted && !has_no_deleted {
                filter.insert("isDeleted", false);
            }
            // If "isActive" is not present and "noActive" is not present, add the default value
            if !has_is_active && !has_no_active {
                filter.insert("isActive", true);
            }
        }

        // Remove "noDeleted" and "noActive" from the filter if they exist
        if has_no_deleted {
            filter.remove("noDeleted");
        }
        if has_no_active {
            filter.remove("noActive");
        }
        println!("Filter: {:?}", filter);
        let collection: &Collection<U> = self.get_collection_for_id();
        let document: mongodb::action::Find<U> = collection.find(filter).allow_disk_use(true).max_await_time(Duration::from_secs(5));
        document
    }

    fn find_generic<'a, J>(
        &'a self,
        mut filter: mongodb::bson::Document,
        collection: &'a Collection<J>,
    ) -> mongodb::action::Find<'_, J>
    where
        J: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        let has_is_deleted = filter.contains_key("isDeleted");
        let has_is_active = filter.contains_key("isActive");
        let has_no_deleted = filter.contains_key("noDeleted");
        let has_no_active = filter.contains_key("noActive");

        // If neither "isDeleted" nor "isActive" are present, add default values
        if !has_is_deleted && !has_is_active && !has_no_deleted && !has_no_active {
            filter.insert("isDeleted", false);
            filter.insert("isActive", true);
        } else {
            // If "isDeleted" is not present and "noDeleted" is not present, add the default value
            if !has_is_deleted && !has_no_deleted {
                filter.insert("isDeleted", false);
            }
            // If "isActive" is not present and "noActive" is not present, add the default value
            if !has_is_active && !has_no_active {
                filter.insert("isActive", true);
            }
        }

        // Remove "noDeleted" and "noActive" from the filter if they exist
        if has_no_deleted {
            filter.remove("noDeleted");
        }
        if has_no_active {
            filter.remove("noActive");
        }
        println!("Filter: {:?}", filter);

        let document: mongodb::action::Find<J> = collection.find(filter);
        document
    }

    fn construct_new_collection<K>(&self) -> Collection<K>
    where
        K: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        let collection = self
            .get_client()
            .database(T::get_database_name())
            .collection(T::get_collection_name());
        collection
    }
}

pub trait Repository<
    T: Serialize + std::marker::Sync + std::marker::Send,
    U: Serialize + std::marker::Sync + std::marker::Send,
>
{
    fn get_client(&self) -> &Client;
    fn get_collection(&self) -> &Collection<T>;
    fn get_collection_for_id(&self) -> &Collection<U>;
}
