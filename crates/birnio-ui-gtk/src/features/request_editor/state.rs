use birnio_core::{Body, Header, HttpMethod};

#[derive(Clone, Debug)]
pub struct State {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub body: Body,
}

impl Default for State {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            url: String::new(),
            headers: Vec::new(),
            body: Body::Empty,
        }
    }
}
