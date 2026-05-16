use birnio_core::Response;

#[derive(Clone, Debug)]
pub enum Message {
    Display(Response),
    Clear,
}
