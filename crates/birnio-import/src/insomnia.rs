use birnio_core::Collection;

use crate::{ImportError, ImportResult};

pub fn import_insomnia_collection(_source: &str) -> ImportResult<Collection> {
    Err(ImportError::UnsupportedFormat("insomnia"))
}
