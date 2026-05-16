mod auth;
mod collection;
mod environment;
mod error;
mod request;
mod response;
mod variable;

pub use auth::{Auth, BasicAuth, BearerAuth};
pub use collection::{Collection, RequestNode};
pub use environment::Environment;
pub use error::{CoreError, CoreResult};
pub use request::{Body, Header, HttpMethod, Request};
pub use response::Response;
pub use variable::Variable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_request_roundtrip() {
        let request = Request::new("Health", HttpMethod::Get, "https://example.com/health")
            .expect("valid request");

        let json = serde_json::to_string(&request).expect("serialize request");
        let decoded: Request = serde_json::from_str(&json).expect("deserialize request");

        assert_eq!(decoded.name, "Health");
        assert_eq!(decoded.method, HttpMethod::Get);
        assert_eq!(decoded.url.as_str(), "https://example.com/health");
    }
}
