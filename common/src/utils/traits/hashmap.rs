use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::utils::macros::get_attributes::GetAttributes;


pub trait HashMapToStruct {
    fn try_from_hashmap<T>(&self) -> Result<Option<T>, Vec<String>>
    where
        T: DeserializeOwned + GetAttributes + Default;
}

impl HashMapToStruct for HashMap<String, String> {
    fn try_from_hashmap<T>(&self) -> Result<Option<T>, Vec<String>>
    where
        T: DeserializeOwned + GetAttributes + Default,
    {
        if self.is_empty() {
            return Ok(None);
        }
        let mut errors = Vec::new();

        let attributes = T::get_attributes(&T::default());
        let mut json_map: Map<String, Value> = Map::with_capacity(self.len());
        for (key, value) in self {
            if !attributes.contains_key(key.as_str()) {
                errors.push(format!("no existe el atributo {} en la estructura propuesta", key));
            }
            

            let json_value = match serde_json::from_str(value) {
                Ok(val) => val,
                Err(_) => {
                    if !value.starts_with('"') && !value.ends_with('"') {
                        Value::String(value[0..value.len()].to_string())
                    } else {
                        errors.push(format!(
                            "transoformacion hasmap error en field {} y valor {}",
                            key, value
                        ));
                        continue;
                    }
                }
            };

            json_map.insert(key.clone(), json_value);
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        let result = serde_json::from_value(Value::Object(json_map))
            .map(Some)
            .map_err(|err| vec![err.to_string()]);

        result
    }
}
