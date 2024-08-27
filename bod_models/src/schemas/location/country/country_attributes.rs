use serde::{Deserialize, Serialize};
use validator::Validate;

use super::country::Country;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CountryAttributes {
    pub code: String,
    pub fullname: String,
    pub currency: String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    pub timezones: Vec<String>,
    pub region: String,
    #[serde(rename = "subRegion")]
    pub sub_region: String,
    pub population: i64,
    pub flag: String,
    pub langs: Vec<String>,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}
impl CountryAttributes {
    pub fn new(
        code: String,
        fullname: String,
        currency: String,
        currency_symbol: String,
        timezones: Vec<String>,
        region: String,
        sub_region: String,
        population: i64,
        langs: Vec<String>,
        flag: String,
        is_deleted: bool,
        is_active: bool,
    ) -> Self {
        Self {
            flag,
            code,
            fullname,
            currency,
            currency_symbol,
            timezones,
            region,
            sub_region,
            population,
            langs,
            is_deleted,
            is_active,
        }
    }
}

impl From<Country> for CountryAttributes {
    fn from(value: Country) -> Self {
        Self::new(
            value.code,
            value.fullname,
            value.currency,
            value.currency_symbol,
            value.timezones,
            value.region,
            value.sub_region,
            value.population,
            value.langs,
            value.flag,
            value.is_deleted,
            value.is_active,
        )
    }
}
