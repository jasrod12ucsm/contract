use compilation_procedure::{ToBson, ToDatabaseQuery};
use serde::{Deserialize, Serialize};
use bod_models::shared::bson::to_document::ToDocument;
use bson::{Bson,to_bson};

#[derive(Debug, Serialize, Deserialize, ToDatabaseQuery)]
pub struct PushLoginByTokenResetToken {
    pub devices: Vec<PushLoginByTokenResetTokenDevice>,
}


#[derive(Debug, Serialize, Deserialize, Clone,ToBson)]
pub struct PushLoginByTokenResetTokenDevice {
    pub os: String,
    pub mac: String,
    pub token: String,
}
