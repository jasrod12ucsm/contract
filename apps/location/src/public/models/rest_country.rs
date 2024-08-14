use bod_models::schemas::location::country::country::Country;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use ntex::util::HashMap;

pub type RestCountries = Vec<RestCountry>;
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestCountry {
    #[serde(default)]
    name: Name,
    #[serde(default)]
    tld: Vec<String>,
    #[serde(default)]
    cca2: String,
    #[serde(default)]
    ccn3: String,
    #[serde(default)]
    cca3: String,
    #[serde(default)]
    cioc: String,
    #[serde(default)]
    independent: bool,
    #[serde(default)]
    status: String,
    #[serde(default)]
    un_member: bool,
    currencies: HashMap<String, Currencies>,
    #[serde(default)]
    idd: Idd,
    #[serde(default)]
    capital: Vec<String>,
    #[serde(default)]
    alt_spellings: Vec<String>,
    #[serde(default)]
    region: String,
    #[serde(default)]
    subregion: String,
    #[serde(default)]
    languages: HashMap<String, String>,
    #[serde(default)]
    translations: HashMap<String, Translation>,
    #[serde(default)]
    latlng: Vec<f64>,
    #[serde(default)]
    landlocked: bool,
    #[serde(default)]
    borders: Vec<String>,
    #[serde(default)]
    area: f64,
    #[serde(default)]
    demonyms: Demonyms,
    #[serde(default)]
    flag: String,
    #[serde(default)]
    maps: Maps,
    #[serde(default)]
    population: i64,
    #[serde(default)]
    gini: Gini,
    #[serde(default)]
    fifa: String,
    #[serde(default)]
    car: Car,
    #[serde(default)]
    timezones: Vec<String>,
    #[serde(default)]
    continents: Vec<String>,
    #[serde(default)]
    flags: Flags,
    #[serde(default)]
    coat_of_arms: CoatOfArms,
    #[serde(default)]
    start_of_week: String,
    #[serde(default)]
    capital_info: CapitalInfo,
    #[serde(default)]
    postal_code: PostalCode,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct CapitalInfo {
    #[serde(default)]
    latlng: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Car {
    #[serde(default)]
    signs: Vec<String>,
    #[serde(default)]
    side: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct CoatOfArms {
    #[serde(default)]
    png: String,
    #[serde(default)]
    svg: String,
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Currencies {
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Pen {
    #[serde(default)]
    name: String,
    #[serde(default)]
    symbol: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Demonyms {
    #[serde(default)]
    eng: Eng,
    #[serde(default)]
    fra: Eng,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Eng {
    #[serde(default)]
    f: String,
    #[serde(default)]
    m: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Flags {
    #[serde(default)]
    png: String,
    #[serde(default)]
    svg: String,
    #[serde(default)]
    alt: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Gini {
    #[serde(rename = "2019")]
    #[serde(default)]
    the_2019: f64,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Idd {
    #[serde(default)]
    root: String,
    #[serde(default)]
    suffixes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Languages {
    #[serde(default)]
    aym: String,
    #[serde(default)]
    que: String,
    #[serde(default)]
    spa: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
#[serde(rename_all = "camelCase")]
pub struct Maps {
    #[serde(default)]
    google_maps: String,
    #[serde(default)]
    open_street_maps: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(default)]
    common: String,
    #[serde(default)]
    official: String,
    #[serde(default)]
    native_name: HashMap<String,Translation>,
}


#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct Translation {
    #[serde(default)]
    official: String,
    #[serde(default)]
    common: String,
}

#[derive(Serialize, Deserialize, Clone,Debug,Default)]
pub struct PostalCode {
    #[serde(default)]
    format: String,
    #[serde(default)]
    regex: String,
}

impl From<&RestCountry> for Country {
    fn from(value: &RestCountry) -> Self {
        let langs: Vec<_> = value.languages.values().map(String::clone).collect();

        let (currency, currency_symbol) = value
            .currencies
            .iter()
            .next()
            .map(|(_, currency_detail)| {
                (currency_detail.name.clone(), currency_detail.symbol.clone())
            })
            .unwrap_or_default();

        Country {
            code: value.cca2.to_owned(),
            fullname: value.name.common.to_owned(),
            region: value.region.to_owned(),
            sub_region: value.subregion.to_owned(),
            population: value.population,
            flag: value.flag.to_owned(),
            currency,
            currency_symbol,
            timezones: value.timezones.to_owned(),
            langs,
            is_delete: false,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
