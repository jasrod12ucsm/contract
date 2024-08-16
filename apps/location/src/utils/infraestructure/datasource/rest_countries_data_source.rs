use common::public::errors::request_error::RequestError;
use reqwest::Client;
use serde_json::from_str;

use crate::{public::models::rest_country::RestCountries, utils::domain::rest_countries_datasource_trait::RestCountriesDataSourceTrait};

pub struct RestCountriesDataSource {
    base_url: String,
    client: Client,
}


#[async_trait::async_trait]
impl RestCountriesDataSourceTrait for  RestCountriesDataSource {
    fn new() -> Self {
        Self {
            base_url: "https://restcountries.com/v3.1".to_string(),
            client: Client::new(),
        }
    }

    async fn get_all_countries(&self) -> Result<RestCountries, reqwest::Error> {
        let response = self
            .client
            .get(self.base_url.clone() + "/all")
            .send()
            .await?;

        let countries: RestCountries = response.json().await?;

        Ok(countries)
    }
    async fn get_country_by_name(&self, name: &str) -> Result<RestCountries, RequestError> {
        let url = self.base_url.clone() + "/name/" + name + "?fullText=true";
        println!("{}", url);
        let response = self.client.get(url).send().await?;
        println!("{:?}", response.status());
        let countries = response.text().await.map_err(|err| {
            RequestError::Reqwest(err)
        })?;
        let countries_json:RestCountries= from_str(&countries).map_err(|err| RequestError::Serde(err))?;
        Ok(countries_json)
    }
}
