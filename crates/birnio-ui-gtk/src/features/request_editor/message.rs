use birnio_core::{Body, Header, HttpMethod};

#[derive(Clone, Debug)]
pub enum Message {
    SetMethod(HttpMethod),
    SetUrl(String),
    SetHeaders(Vec<Header>),
    SetBody(Body),
    Send,
}
