use birnio_core::Collection;
use serde::Deserialize;

use crate::ImportResult;

#[derive(Debug, Deserialize)]
struct PostmanCollection {
    info: PostmanInfo,
}

#[derive(Debug, Deserialize)]
struct PostmanInfo {
    name: String,
}

pub fn import_postman_collection(source: &str) -> ImportResult<Collection> {
    let postman: PostmanCollection = serde_json::from_str(source)?;
    Ok(Collection::new(postman.info.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_minimal_postman_collection_name() {
        let collection =
            import_postman_collection(r#"{ "info": { "name": "Example API" } }"#).unwrap();

        assert_eq!(collection.name, "Example API");
        assert!(collection.nodes.is_empty());
    }
}
