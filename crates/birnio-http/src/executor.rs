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
    async fn execute(&self, request: &Request) -> HttpResult<Response> {
        let reqwest_request = request_builder::build(self.client.inner(), request)?;
        let started_at = Instant::now();
        let response = self.client.inner().execute(reqwest_request).await?;

        response_parser::parse(response, started_at.elapsed()).await
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use birnio_core::HttpMethod;

    use super::*;

    #[tokio::test]
    async fn executes_request_against_local_server() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("read local address");

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer).expect("read request");
            stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nok",
                )
                .expect("write response");
        });

        let request = Request::new("Local", HttpMethod::Get, format!("http://{address}/"))
            .expect("valid request");
        let executor = ReqwestExecutor::new(HttpClient::new().expect("http client"));

        let response = executor.execute(&request).await.expect("http response");

        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"ok");
    }
}
