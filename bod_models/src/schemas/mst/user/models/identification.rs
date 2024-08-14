use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Identification{
    #[serde(rename="identificationNumber")]
    pub identification_number:String,//*= 1234567890*/
    #[serde(rename="identificationType")]
    pub identification_type:String,//*= RUC,DNI,RUS*/
}