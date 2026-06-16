use std::{future::Future, time::Instant};

use birnio_core::{Request, Response};

use crate::{HttpClient, HttpResult, request_builder, response_parser};

pub trait HttpExecutor {
    fn execute(&self, request: &Request) -> impl Future<Output = HttpResult<Response>> + Send;
}

#[derive(Clone)]
pub struct ReqwestExecutor {
    client: HttpClient,
}

impl ReqwestExecutor {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl HttpExecutor for ReqwestExecutor {
    // `async fn` would trigger `refining_impl_trait` because the trait uses RPIT with `+ Send`.
    #[allow(clippy::manual_async_fn)]
    fn execute(&self, request: &Request) -> impl Future<Output = HttpResult<Response>> + Send {
        async move {
            let reqwest_request = request_builder::build(self.client.inner(), request)?;
            let started_at = Instant::now();
            let response = self.client.inner().execute(reqwest_request).await?;

            response_parser::parse(response, started_at.elapsed()).await
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
        time::Duration,
    };

    use birnio_core::{Body, Header, HttpMethod};

    use super::*;

    const OK_RESPONSE: &[u8] =
        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nok";

    #[tokio::test]
    async fn executes_request_against_local_server() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("read local address");

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer).expect("read request");
            stream.write_all(OK_RESPONSE).expect("write response");
        });

        let request = Request::new("Local", HttpMethod::Get, format!("http://{address}/"))
            .expect("valid request");
        let executor = ReqwestExecutor::new(HttpClient::new().expect("http client"));

        let response = executor.execute(&request).await.expect("http response");

        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"ok");
    }

    #[tokio::test]
    async fn sends_headers_in_request() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("read local address");
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).expect("read request");
            let raw = String::from_utf8_lossy(&buffer[..n]);
            assert!(
                raw.to_lowercase().contains("x-custom: test-value"),
                "header not found in request:\n{raw}"
            );
            stream.write_all(OK_RESPONSE).expect("write response");
        });

        let executor = ReqwestExecutor::new(HttpClient::new().expect("http client"));
        let mut request = Request::new("Headers", HttpMethod::Get, format!("http://{address}/"))
            .expect("valid request");
        request.headers.push(Header {
            name: "X-Custom".into(),
            value: "test-value".into(),
            enabled: true,
        });

        let response = executor.execute(&request).await.expect("http response");
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn sends_text_body_in_request() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("read local address");

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).expect("read request");
            let raw = String::from_utf8_lossy(&buffer[..n]);
            assert!(
                raw.contains("hello world"),
                "text body not found in:\n{raw}"
            );
            stream.write_all(OK_RESPONSE).expect("write response");
        });

        let executor = ReqwestExecutor::new(HttpClient::new().expect("http client"));
        let mut request = Request::new("Body", HttpMethod::Post, format!("http://{address}/"))
            .expect("valid request");
        request.body = Body::Text("hello world".into());

        let response = executor.execute(&request).await.expect("http response");
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn sends_json_body_in_request() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("read local address");

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).expect("read request");
            let raw = String::from_utf8_lossy(&buffer[..n]);
            assert!(
                raw.to_lowercase()
                    .contains("content-type: application/json"),
                "json content-type missing in:\n{raw}"
            );
            assert!(
                raw.contains(r#""key":"value""#),
                "json body not found in:\n{raw}"
            );
            stream.write_all(OK_RESPONSE).expect("write response");
        });

        let executor = ReqwestExecutor::new(HttpClient::new().expect("http client"));
        let mut request = Request::new("Json", HttpMethod::Post, format!("http://{address}/"))
            .expect("valid request");
        request.body = Body::Json(serde_json::json!({"key": "value"}));

        let response = executor.execute(&request).await.expect("http response");
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn parses_response_with_all_fields() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("read local address");

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer).expect("read request");

            stream
                .write_all(
                    b"HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nX-Request-Id: abc\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}",
                )
                .expect("write response");
        });

        let request = Request::new("Parse", HttpMethod::Get, format!("http://{address}/"))
            .expect("valid request");
        let executor = ReqwestExecutor::new(HttpClient::new().expect("http client"));
        let response = executor.execute(&request).await.expect("http response");

        assert_eq!(response.status, 201);
        assert!(
            response.headers.iter().any(|h| h.name == "content-type"),
            "expected content-type header in response"
        );
        assert!(
            response.headers.iter().any(|h| h.name == "x-request-id"),
            "expected x-request-id header in response"
        );
        assert_eq!(response.body, b"{\"status\":\"ok\"}");
        assert!(
            response.elapsed > Duration::from_millis(0),
            "expected non-zero elapsed time"
        );
    }
}
