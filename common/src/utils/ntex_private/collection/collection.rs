use async_trait::async_trait;
use bod_models::shared::schema::Schema;
use mongodb::Client;


pub struct Collection<'b> {
    collections: Vec<Box<dyn Schema + Send + Sync>>,
    client: &'b Client,
}

impl<'a> Collection<'a> {
    pub fn new(client: &'a Client, collections: Vec<Box<dyn Schema + Send + Sync>>) -> Self {
        Self {
            collections,
            client,
        }
    }
}

#[async_trait]
pub trait CollectionFunctions {
    async fn run_indexes(&self) -> ();
}
#[async_trait]
impl<'b> CollectionFunctions for Collection<'b> {
    async fn run_indexes(&self) {
        for collection in &self.collections {
            let _ = collection.set_indexes(self.client).await.map_err(|err| {
                println!("error in index: {:?}", err);
                panic!("error in index")
            });
        }
    }
}
