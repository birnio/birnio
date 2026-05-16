use thiserror::Error;

pub type HttpResult<T> = Result<T, HttpError>;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("failed to build HTTP request: {0}")]
    RequestBuild(String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
