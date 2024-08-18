use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize,Deserialize,Debug,Default,Validate)]
pub struct RegisterUserClientDto {
    pub names: String,
    pub surnames: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min=12))]
    pub password: String,
    #[serde(rename="identificationNumber")]
    pub identification_number: String,
    #[serde(rename="identificationType")]
    pub identification_type: String,
    pub phone:String,
    pub address:String,
    #[serde(rename="countryCode")]
    pub country_code:String,
    #[serde(rename="regionCode")]
    pub region_code:String,
    pub birthdate:String
}
