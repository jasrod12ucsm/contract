use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct AtentionHour {
    #[builder(default)]
    pub monday: Option<String>,
    #[builder(default)]
    pub tuesday: Option<String>,
    #[builder(default)]
    pub wednesday: Option<String>,
    #[builder(default)]
    pub thursday: Option<String>,
    #[builder(default)]
    pub friday: Option<String>,
    #[builder(default)]
    pub saturday: Option<String>,
    #[builder(default)]
    pub sunday: Option<String>,
}

impl AtentionHourBuilder {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = bson::Document::new();
        if let Some(monday) = &self.monday {
            doc.insert("monday", monday);
        }
        if let Some(tuesday) = &self.tuesday {
            doc.insert("tuesday", tuesday);
        }
        if let Some(wednesday) = &self.wednesday {
            doc.insert("wednesday", wednesday);
        }
        if let Some(thursday) = &self.thursday {
            doc.insert("thursday", thursday);
        }
        if let Some(friday) = &self.friday {
            doc.insert("friday", friday);
        }
        if let Some(saturday) = &self.saturday {
            doc.insert("saturday", saturday);
        }
        if let Some(sunday) = &self.sunday {
            doc.insert("sunday", sunday);
        }
        doc
    }
}

impl AtentionHour {
    pub fn create_empty() -> Self {
        AtentionHour {
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
