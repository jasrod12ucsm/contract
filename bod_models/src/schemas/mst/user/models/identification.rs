use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Identification{
    pub identification_number:String,//*= 1234567890*/
    pub identification_type:String,//*= RUC,DNI,RUS*/
}