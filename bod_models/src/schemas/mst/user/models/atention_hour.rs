use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AtentionHour{
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub saturday: Option<String>,
    pub sunday: Option<String>,
}
impl AtentionHour{
    pub fn create_empty() -> Self{
        AtentionHour{
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None,
            saturday: None,
            sunday: None,
        }
    }
}