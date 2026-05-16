use birnio_core::Response;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub response: Option<Response>,
}
