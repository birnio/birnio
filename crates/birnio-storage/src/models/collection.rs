use birnio_core::{Collection, RequestNode};
use uuid::Uuid;

use crate::StorageResult;

#[derive(Clone, Debug)]
pub struct CollectionRecord {
    pub id: String,
    pub name: String,
    pub nodes_json: String,
}

impl CollectionRecord {
    pub fn from_domain(collection: &Collection) -> StorageResult<Self> {
        Ok(Self {
            id: collection.id.to_string(),
            name: collection.name.clone(),
            nodes_json: serde_json::to_string(&collection.nodes)?,
        })
    }

    pub fn into_domain(self) -> StorageResult<Collection> {
        Ok(Collection {
            id: Uuid::parse_str(&self.id)?,
            name: self.name,
            nodes: serde_json::from_str::<Vec<RequestNode>>(&self.nodes_json)?,
        })
    }
}
