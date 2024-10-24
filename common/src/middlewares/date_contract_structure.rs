use bod_models::shared::jwt::claims::DefaultClaims;
use bson::oid::ObjectId;

#[derive(Clone,Debug)]
pub struct DateContractStructure{
    pub start_date: String,
    pub finish_date: String,
    pub id:ObjectId
}




impl From<DefaultClaims> for DateContractStructure{
    fn from(value: DefaultClaims) -> Self {
        DateContractStructure{
            id: value.id().clone(),
            start_date: value.start().to_string(),
            finish_date: value.finish().to_string(),
        }
    }
}