use std::time::Duration;

use async_trait::async_trait;
use mongodb::{
    action::Update, bson::{doc, DateTime}, error::Error, options::{
        ClientOptions, Compressor, ReadPreference, ReadPreferenceOptions, ReturnDocument, SelectionCriteria, ServerApiVersion
    }, results::InsertOneResult, Client, ClientSession, Collection
};
use serde::{de::DeserializeOwned, Serialize};

use crate::helpers::env::env::ENV;
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

#[async_trait]
pub trait AbstractRepository<T: Serialize + Send + Sync, U: Serialize + Send + Sync> {
    async fn insert_one(
        &self,
        item: T,
        session: Option<&mut ClientSession>,
    ) -> Result<InsertOneResult, Error>;
    async fn find_one(
        &self,
        filter: mongodb::bson::Document,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<U>, Error>;
    async fn find_one_and_update(
        &self,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<U>, Error>;
    async fn find_one_and_update_with_upsert(
        &self,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<U>, Error>;
    fn update_one(
        &self,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
    ) -> Update;
    fn get_all(&self) -> mongodb::action::Find<'_, U>;
    fn find(&self, filter: mongodb::bson::Document) -> mongodb::action::Find<'_, U>;
}

#[async_trait]
impl<T, U, R> AbstractRepository<T, U> for R
where
    T: Serialize + Send + Sync + 'static + Unpin + DeserializeOwned,
    R: Repository<T, U> + Send + Sync,
    U: Serialize + Send + Sync + 'static + Unpin + DeserializeOwned,
{
    async fn insert_one(
        &self,
        item: T,
        session: Option<&mut ClientSession>,
    ) -> Result<InsertOneResult, Error> {
        let collection = self.get_collection();
        if let Some(session) = session {
            return collection.insert_one(item).session(session).await;
        }
        collection.insert_one(item).await
    }
    async fn find_one(
        &self,
        filter: mongodb::bson::Document,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<U>, Error> {
        let collection = self.get_collection_for_id();
        if let Some(session) = session {
            let doc = collection.find_one(filter).selection_criteria(SelectionCriteria::ReadPreference(ReadPreference::Primary)).session(session).await?;
            return Ok(doc);
        }
        let document = collection.find_one(filter).await?;
        Ok(document)
    }
    //findOneandUpdate
    async fn find_one_and_update(
        &self,
        filter: mongodb::bson::Document,
        mut update: mongodb::bson::Document,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<U>, Error> {
        let collection = self.get_collection_for_id();
        //pon el updated at en el update
        let now=DateTime::now();
        if let Ok(set_doc)=update.get_document_mut("$set"){
            set_doc.insert("updatedAt", now);
        }else{
            update.insert("$set", doc! {"updatedAt": now});
        }
        if let Some(session) = session {
            let doc = collection
                .find_one_and_update(filter, update)
                .session(session)
                .await?;
            return Ok(doc);
        }
        let document = collection.find_one_and_update(filter, update).await?;
        Ok(document)
    }
    async fn find_one_and_update_with_upsert(
        &self,
        filter: mongodb::bson::Document,
        mut update: mongodb::bson::Document,
        session: Option<&mut ClientSession>,
    ) -> Result<Option<U>, Error> {
        let collection = self.get_collection_for_id();
        let now = DateTime::now();
        if let Ok(set_doc) = update.get_document_mut("$set") {
            set_doc.insert("updatedAt", now);
        } else {
            update.insert("$set", doc! {"updatedAt": now});
        }
        update.insert("$setOnInsert", doc! {"createdAt": now});
        if let Some(session) = session {
            let doc = collection
                .find_one_and_update(filter, update)
                .upsert(true)
                .return_document(ReturnDocument::After)
                .session(session)
                .await?;
            return Ok(doc);
        }
        let document = collection
            .find_one_and_update(filter, update)
            .upsert(true)
            .return_document(ReturnDocument::After)
            .await?;
        Ok(document)
    }
    fn update_one(
        &self,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
    ) -> Update {
        let collection = self.get_collection_for_id();
        let document = collection.update_one(filter, update);
        document
    }
    fn get_all(&self) -> mongodb::action::Find<'_, U> {
        let collection = self.get_collection_for_id();
        let document = collection.find(doc! {});
        document
    }

    fn find(&self, filter: mongodb::bson::Document) -> mongodb::action::Find<'_,U>{
        let collection = self.get_collection_for_id();
        let document = collection.find(filter);
        document
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
