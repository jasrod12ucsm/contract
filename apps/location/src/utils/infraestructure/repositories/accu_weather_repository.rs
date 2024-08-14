use common::public::errors::request_error::RequestError;

use crate::{public::models::accu_region::AccuRegions, utils::infraestructure::datasource::accu_wheather_data_source::AccuWheatherDataSource};

pub struct AccuWeatherRepository{
    datasource:AccuWheatherDataSource
}

impl AccuWeatherRepository{
    pub fn new(datasource:AccuWheatherDataSource) -> Self{
        Self{
            datasource
        }
    }

    pub async fn get_all_regions(&self,country_name:String) -> Result<AccuRegions,RequestError >{
        let countries = self.datasource.get_regions(country_name.as_str()).await?;
        Ok(countries)
    }
}