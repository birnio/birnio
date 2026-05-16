use birnio_core::Request;
use uuid::Uuid;

use crate::StorageResult;

#[derive(Clone, Debug)]
pub struct RequestRecord {
    pub id: String,
    pub collection_id: Option<String>,
    pub name: String,
    pub request_json: String,
}

impl RequestRecord {
    pub fn from_domain(request: &Request, collection_id: Option<Uuid>) -> StorageResult<Self> {
        Ok(Self {
            id: request.id.to_string(),
            collection_id: collection_id.map(|id| id.to_string()),
            name: request.name.clone(),
            request_json: serde_json::to_string(request)?,
        })
    }

    pub fn into_domain(self) -> StorageResult<Request> {
        Ok(serde_json::from_str::<Request>(&self.request_json)?)
    }
}
