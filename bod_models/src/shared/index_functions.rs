use std::collections::HashMap;

use futures::StreamExt;
use mongodb::{Collection, IndexModel};

pub struct IndexFunctions;

impl IndexFunctions {
    pub async fn delete_existing_indexes<T: Sync + Send>(
        collection: &Collection<T>,
        indexes: &mut Vec<IndexModel>,
    ) -> Result<(), mongodb::error::Error> {
        let mut names_hash: HashMap<String, usize> = HashMap::new();

        for (i, name) in indexes.iter().enumerate() {
            if let Some(options) = &name.options {
                if let Some(name_str) = &options.name {
                    names_hash.insert(name_str.clone(), i);
                }
            }
        }
        let mut indexes_to_remove = Vec::new();

        // Obtener el cursor de índices
        let mut indexes_cursor = collection.list_indexes().await?;
        while let Some(index) = indexes_cursor.next().await {
            let index = index?;
            if let Some(index_name) = index
                .options
                .as_ref()
                .and_then(|options| options.name.as_deref())
            {
                if let Some(&i) = names_hash.get(index_name) {
                    indexes_to_remove.push(i);
                }
            }
        }

        // Eliminar los índices del vector original
        indexes_to_remove.sort_unstable();
        for i in indexes_to_remove.iter().rev() {
            indexes.remove(*i);
        }
        Ok(())
    }
}
