use derive_builder::Builder;
use serde::{Deserialize, Serialize};



use super::country_with_id::CountryWithId;

#[derive(Debug, Clone, Serialize, Deserialize, Builder,Default)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct ShortCountry {
    pub code: String,
    pub fullname: String,
    pub currency: String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl ShortCountryBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.code.is_none() {
            return Err("code is required".into());
        }
        if self.fullname.is_none() {
            return Err("fullname is required".into());
        }
        if self.currency.is_none() {
            return Err("currency is required".into());
        }
        if self.currency_symbol.is_none() {
            return Err("currency_symbol is required".into());
        }
        if self.is_active.is_none() {
            return Err("is_active is required".into());
        }
        Ok(())
    }

    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = bson::Document::new();
        if let Some(code) = &self.code {
            doc.insert("code", code);
        }
        if let Some(fullname) = &self.fullname {
            doc.insert("fullname", fullname);
        }
        if let Some(currency) = &self.currency {
            doc.insert("currency", currency);
        }
        if let Some(currency_symbol) = &self.currency_symbol {
            doc.insert("currencySymbol", currency_symbol);
        }
        if let Some(is_active) = &self.is_active {
            doc.insert("isActive", is_active);
        }
        doc
    }
}


impl From<CountryWithId> for ShortCountry {
    fn from(value: CountryWithId) -> Self {
        Self {
            code: value.code,
            fullname: value.fullname,
            currency: value.currency,
            currency_symbol: value.currency_symbol,
            is_active: value.is_active,
        }
    }
}