use serde::{Deserialize, Serialize};

use super::country_with_id::CountryWithId;

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ShortCountry {
    pub code: String,
    pub fullname: String,
    pub currency: String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
     #[serde(rename = "isDelete")]
    pub is_delete: bool,
}

impl From<CountryWithId> for ShortCountry{
    fn from(value: CountryWithId) -> Self {
        ShortCountry {
            code: value.code,
            fullname: value.fullname,
            currency: value.currency,
            currency_symbol: value.currency_symbol,
            is_delete: value.is_delete,
        }
    }
}