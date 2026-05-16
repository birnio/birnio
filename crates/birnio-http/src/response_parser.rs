use std::time::Duration;

use birnio_core::{Header, Response};

use crate::HttpResult;

pub(crate) async fn parse(response: reqwest::Response, elapsed: Duration) -> HttpResult<Response> {
    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| Header {
            name: name.to_string(),
            value: value.to_str().unwrap_or_default().to_owned(),
            enabled: true,
        })
        .collect();
    let body = response.bytes().await?.to_vec();

    Ok(Response {
        status,
        headers,
        body,
        elapsed,
    })
}
