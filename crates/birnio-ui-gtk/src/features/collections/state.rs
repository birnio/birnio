use birnio_core::Collection;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub collections: Vec<Collection>,
    pub selected_collection_id: Option<Uuid>,
    pub selected_request_id: Option<Uuid>,
}
