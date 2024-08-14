use async_trait::async_trait;
use mongodb::{results::CreateIndexesResult, Client};


#[async_trait]
pub trait Schema:Sync+Send{
    fn get_collection_name(&self)->&'static str;
    fn get_database_name(&self)-> &'static str;
    async fn set_indexes(&self,client:&Client)-> Result<Option<CreateIndexesResult>,mongodb::error::Error>;
}
pub trait BaseColleccionNames {
   fn get_collection_name() -> &'static str;

    fn get_database_name() -> &'static str;
}
