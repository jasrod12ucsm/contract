
use bod_models::{schemas::config::email_template::{email_template::EmailTemplate, models::email_template_with_id::EmailTemplateWithId}, shared::schema::BaseColleccionNames};
use common::utils::ntex_private::repository::public_repository::{
    PublicRepository, Repository, SetPublicRepository,
};
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use std::sync::{Arc, Mutex};

lazy_static! (
    pub static ref EMAIL_TEMPLATE_REPOSITORY: Arc<Mutex<Option<EmailTemplateRepository>>> =
        Arc::new(Mutex::new(None));
);

#[derive(Clone)]
pub struct EmailTemplateRepository {
    collection: Collection<EmailTemplate>,
    collection_id: Collection<EmailTemplateWithId>,
    client: Client,
}

impl Repository<EmailTemplate, EmailTemplateWithId> for EmailTemplateRepository {
    fn get_collection(&self) -> &Collection<EmailTemplate> {
        &self.collection
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_collection_for_id(&self) -> &Collection<EmailTemplateWithId> {
        &self.collection_id
    }
}

impl EmailTemplateRepository {
    pub async fn new(repository: &PublicRepository) -> Result<Self, mongodb::error::Error> {
        let client = repository.get_client()?;
        let collection: Collection<EmailTemplate> = client
            .database(EmailTemplate::get_database_name())
            .collection(EmailTemplate::get_collection_name());
        let collection_id: Collection<EmailTemplateWithId> = client
            .database(EmailTemplate::get_database_name())
            .collection(EmailTemplate::get_collection_name());
        Ok(EmailTemplateRepository {
            collection,
            client: client.clone(),
            collection_id,
        })
    }
}

#[async_trait::async_trait]
impl SetPublicRepository for EmailTemplateRepository {
    type RepositoryType = EmailTemplateRepository;

    async fn set_repository(
        repository: &PublicRepository,
    ) -> Result<Self::RepositoryType, mongodb::error::Error> {
        let repository_option = {
            let value = &EMAIL_TEMPLATE_REPOSITORY;

            if value.lock().unwrap().is_none() {
                let obj_repository = EmailTemplateRepository::new(repository).await?;
                *(value.lock().unwrap()) = Some(obj_repository);
            }

            value
        };
        let repository_option = repository_option.lock().unwrap().clone().unwrap();
        Ok(repository_option)
    }
}
