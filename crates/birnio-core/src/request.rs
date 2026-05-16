use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{Auth, CoreError, CoreResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: Url,
    pub headers: Vec<Header>,
    pub body: Body,
    pub auth: Auth,
}

impl Request {
    pub fn new(
        name: impl Into<String>,
        method: HttpMethod,
        url: impl AsRef<str>,
    ) -> CoreResult<Self> {
        let url =
            Url::parse(url.as_ref()).map_err(|_| CoreError::InvalidUrl(url.as_ref().to_owned()))?;

        Ok(Self {
            id: Uuid::new_v4(),
            name: name.into(),
            method,
            url,
            headers: Vec::new(),
            body: Body::Empty,
            auth: Auth::None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Body {
    Empty,
    Text(String),
    Json(serde_json::Value),
}
