use birnio_core::Collection;

use crate::{ImportError, ImportResult};

pub fn import_bruno_collection(_source: &str) -> ImportResult<Collection> {
    Err(ImportError::UnsupportedFormat("bruno"))
}
