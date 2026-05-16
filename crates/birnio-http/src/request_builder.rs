use birnio_core::{Auth, Body, Header, HttpMethod, Request};

use crate::{HttpError, HttpResult};

pub(crate) fn build(client: &reqwest::Client, request: &Request) -> HttpResult<reqwest::Request> {
    let method = to_reqwest_method(&request.method);
    let mut builder = client.request(method, request.url.clone());

    for Header { name, value, .. } in request.headers.iter().filter(|header| header.enabled) {
        builder = builder.header(name, value);
    }

    builder = match &request.auth {
        Auth::None => builder,
        Auth::Basic(auth) => builder.basic_auth(&auth.username, Some(&auth.password)),
        Auth::Bearer(auth) => builder.bearer_auth(&auth.token),
    };

    builder = match &request.body {
        Body::Empty => builder,
        Body::Text(value) => builder.body(value.clone()),
        Body::Json(value) => builder.json(value),
    };

    builder
        .build()
        .map_err(|error| HttpError::RequestBuild(error.to_string()))
}

fn to_reqwest_method(method: &HttpMethod) -> reqwest::Method {
    match method {
        HttpMethod::Get => reqwest::Method::GET,
        HttpMethod::Post => reqwest::Method::POST,
        HttpMethod::Put => reqwest::Method::PUT,
        HttpMethod::Patch => reqwest::Method::PATCH,
        HttpMethod::Delete => reqwest::Method::DELETE,
        HttpMethod::Head => reqwest::Method::HEAD,
        HttpMethod::Options => reqwest::Method::OPTIONS,
    }
}
