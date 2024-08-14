use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct EmailSended{
    pub ok:bool
}