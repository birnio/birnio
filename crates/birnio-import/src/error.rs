use thiserror::Error;

pub type ImportResult<T> = Result<T, ImportError>;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("import format is not implemented yet: {0}")]
    UnsupportedFormat(&'static str),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
