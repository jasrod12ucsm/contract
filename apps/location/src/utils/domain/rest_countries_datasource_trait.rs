use common::public::errors::request_error::RequestError;

use crate::public::models::rest_country::RestCountries;


#[async_trait::async_trait]
pub trait RestCountriesDataSourceTrait {
    fn new() -> Self;
    async fn get_all_countries(&self) -> Result<RestCountries, reqwest::Error>;
    async fn get_country_by_name(&self, name: &str) -> Result<RestCountries, RequestError>;
}
