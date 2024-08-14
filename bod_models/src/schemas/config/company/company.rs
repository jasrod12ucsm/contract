use serde::{Deserialize, Serialize};

use crate::{schemas::{location::{country::models::short_country::ShortCountry, region::region::Region}, mst::user::models::short_user::ShortUser}, shared::schema::BaseColleccionNames};



#[derive(Serialize,Deserialize)]
pub struct Company{
    logo:String,
    #[serde(rename="mediumLogo")]
    medium_logo:String,
    emails:Vec<String>,
    #[serde(rename= "whatsAppNumbers")]
    whats_app_numbers:Vec<String>,
    company_type:String,// F o S
    #[serde(rename="paymentPrice")]
    payment_price:String,
    name:String,
    #[serde(rename="dispĺayName")]
    display_name:String,
    permissions:String,
    user:ShortUser,
    country:ShortCountry,    
    region:Region
}


pub struct CompanySchema;

impl BaseColleccionNames for Company{
    fn get_collection_name() -> &'static str {
        "cnf-company"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}






