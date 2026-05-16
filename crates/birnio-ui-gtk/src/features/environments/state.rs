use birnio_core::Environment;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub environments: Vec<Environment>,
    pub selected_environment_id: Option<Uuid>,
}
