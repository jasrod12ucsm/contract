use common::public::errors::request_error::RequestError;

use crate::{public::models::rest_country::RestCountries, utils::{domain::rest_countries_datasource_trait::RestCountriesDataSourceTrait, infraestructure::datasource::rest_countries_data_source::RestCountriesDataSource}};

pub struct RestCountriesRepository{
    datasource:RestCountriesDataSource
}

impl RestCountriesRepository{
    pub fn new(datasource:RestCountriesDataSource) -> Self{
        Self{
            datasource
        }
    }

    pub async fn get_all_countries(&self) -> Result<RestCountries,RequestError >{
        let countries = self.datasource.get_all_countries().await?;
        Ok(countries)
    }
    pub async fn get_country_by_name(&self, name:String) -> Result<RestCountries,RequestError>{
        let countries = self.datasource.get_country_by_name(name.as_str()).await?;
        Ok(countries)
    }
}