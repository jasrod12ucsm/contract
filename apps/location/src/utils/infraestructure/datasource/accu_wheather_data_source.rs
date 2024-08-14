use reqwest::Client;

use crate::public::models::accu_region::AccuRegions;

pub struct AccuWheatherDataSource {
    base_url: String,
    client: Client,
}
impl AccuWheatherDataSource {
    pub fn new() -> Self {
        Self {
            base_url: "http://dataservice.accuweather.com/locations/v1/".to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_regions<'a>(&self,country_name:&'a str) -> Result<AccuRegions, reqwest::Error> {
        let response = self
            .client
            .get(self.base_url.clone() + "adminareas/"+country_name+"?apikey=HMLEwsPucRpAAGmqbu9JFhJpi5kEASwx")
            .send()
            .await?;

        let countries: AccuRegions = response.json().await?;

        Ok(countries)
    }
}
