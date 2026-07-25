use birnio_core::{Body, Collection, Header, HttpMethod, Request, RequestNode};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use super::types::{
    BirnioCollection, BirnioHeader, BirnioRequest, HarCreator, HarDocument, HarEntry, HarLog,
    HarNameValue, HarPostData, HarRequest,
};

const CREATOR_NAME: &str = "Birnio";
const CREATOR_VERSION: &str = env!("CARGO_PKG_VERSION");
const HAR_VERSION: &str = "1.2";
const HTTP_VERSION: &str = "HTTP/1.1";

/// Errors produced during HAR ↔ Birnio domain conversion.
#[derive(Debug, Error)]
pub enum HarError {
    #[error("invalid URL '{url}': {reason}")]
    InvalidUrl { url: String, reason: String },
    #[error("unknown HTTP method: {0}")]
    UnknownMethod(String),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type HarResult<T> = Result<T, HarError>;

/// Convert a [`Collection`] into a [`HarDocument`] ready for serialization.
///
/// All requests are flattened into HAR entries regardless of nesting depth.
/// Folder structure is encoded in each entry's `_birnio.folder` path so it
/// can be reconstructed on load. Only enabled headers appear in the standard
/// `request.headers` field; the full set (including disabled) is stored in
/// `_birnio.headers` for round-trip fidelity.
pub fn collection_to_har(collection: &Collection) -> HarDocument {
    let entries = flatten_nodes(&collection.nodes, &[]);

    HarDocument {
        log: HarLog {
            version: HAR_VERSION.to_string(),
            creator: HarCreator {
                name: CREATOR_NAME.to_string(),
                version: CREATOR_VERSION.to_string(),
            },
            entries,
            birnio: BirnioCollection {
                id: collection.id,
                name: collection.name.clone(),
            },
        },
    }
}

/// Convert a [`HarDocument`] back into a [`Collection`].
///
/// Folder structure is reconstructed from each entry's `_birnio.folder` path.
/// Folder UUIDs are freshly generated because HAR does not preserve them.
pub fn har_to_collection(doc: HarDocument) -> HarResult<Collection> {
    let log = doc.log;
    let mut nodes: Vec<RequestNode> = Vec::new();

    for entry in log.entries {
        let folder = entry.birnio.folder.clone();
        let request = har_entry_to_request(entry)?;
        insert_at_path(&mut nodes, RequestNode::Request(request), &folder);
    }

    Ok(Collection {
        id: log.birnio.id,
        name: log.birnio.name,
        nodes,
    })
}

// ── private helpers ──────────────────────────────────────────────────────────

fn flatten_nodes(nodes: &[RequestNode], folder: &[String]) -> Vec<HarEntry> {
    let mut entries = Vec::new();
    for node in nodes {
        match node {
            RequestNode::Request(req) => {
                entries.push(request_to_har_entry(req, folder));
            }
            RequestNode::Folder { name, children, .. } => {
                let mut path = folder.to_vec();
                path.push(name.clone());
                entries.extend(flatten_nodes(children, &path));
            }
        }
    }
    entries
}

fn request_to_har_entry(request: &Request, folder: &[String]) -> HarEntry {
    let enabled_headers: Vec<HarNameValue> = request
        .headers
        .iter()
        .filter(|h| h.enabled)
        .map(|h| HarNameValue {
            name: h.name.clone(),
            value: h.value.clone(),
        })
        .collect();

    let birnio_headers: Vec<BirnioHeader> = request
        .headers
        .iter()
        .map(|h| BirnioHeader {
            name: h.name.clone(),
            value: h.value.clone(),
            enabled: h.enabled,
        })
        .collect();

    let (post_data, body_size) = body_to_post_data(&request.body);

    HarEntry {
        request: HarRequest {
            method: request.method.as_str().to_string(),
            url: request.url.to_string(),
            http_version: HTTP_VERSION.to_string(),
            cookies: Vec::new(),
            headers: enabled_headers,
            query_string: Vec::new(),
            post_data,
            headers_size: -1,
            body_size,
        },
        response: None,
        birnio: BirnioRequest {
            id: request.id,
            name: request.name.clone(),
            auth: request.auth.clone(),
            headers: birnio_headers,
            folder: folder.to_vec(),
        },
    }
}

fn body_to_post_data(body: &Body) -> (Option<HarPostData>, i64) {
    match body {
        Body::Empty => (None, 0),
        Body::Text(text) => (
            Some(HarPostData {
                mime_type: "text/plain".to_string(),
                text: text.clone(),
            }),
            text.len() as i64,
        ),
        Body::Json(value) => {
            let text = serde_json::to_string(value).unwrap_or_default();
            let size = text.len() as i64;
            (
                Some(HarPostData {
                    mime_type: "application/json".to_string(),
                    text,
                }),
                size,
            )
        }
    }
}

fn har_entry_to_request(entry: HarEntry) -> HarResult<Request> {
    let method = parse_method(&entry.request.method)?;
    let url_str = entry.request.url.clone();
    let url: Url = url_str
        .parse()
        .map_err(|e: url::ParseError| HarError::InvalidUrl {
            url: url_str,
            reason: e.to_string(),
        })?;

    let headers: Vec<Header> = entry
        .birnio
        .headers
        .iter()
        .map(|h| Header {
            name: h.name.clone(),
            value: h.value.clone(),
            enabled: h.enabled,
        })
        .collect();

    let body = post_data_to_body(entry.request.post_data)?;

    Ok(Request {
        id: entry.birnio.id,
        name: entry.birnio.name,
        method,
        url,
        headers,
        body,
        auth: entry.birnio.auth,
    })
}

fn parse_method(s: &str) -> HarResult<HttpMethod> {
    match s {
        "GET" => Ok(HttpMethod::Get),
        "POST" => Ok(HttpMethod::Post),
        "PUT" => Ok(HttpMethod::Put),
        "PATCH" => Ok(HttpMethod::Patch),
        "DELETE" => Ok(HttpMethod::Delete),
        "HEAD" => Ok(HttpMethod::Head),
        "OPTIONS" => Ok(HttpMethod::Options),
        other => Err(HarError::UnknownMethod(other.to_string())),
    }
}

fn post_data_to_body(post_data: Option<HarPostData>) -> HarResult<Body> {
    match post_data {
        None => Ok(Body::Empty),
        Some(pd) if pd.mime_type.contains("json") => {
            let value: serde_json::Value = serde_json::from_str(&pd.text)?;
            Ok(Body::Json(value))
        }
        Some(pd) => Ok(Body::Text(pd.text)),
    }
}

/// Insert `node` at the nested folder `path`, creating intermediate folders as needed.
///
/// Existing folders are matched by name. Folder UUIDs are freshly generated
/// when a new folder is created.
fn insert_at_path(nodes: &mut Vec<RequestNode>, node: RequestNode, path: &[String]) {
    if path.is_empty() {
        nodes.push(node);
        return;
    }

    let folder_name = path[0].as_str();
    let rest = &path[1..];

    let folder_idx = nodes
        .iter()
        .position(|n| matches!(n, RequestNode::Folder { name, .. } if name == folder_name));

    if let Some(idx) = folder_idx {
        if let RequestNode::Folder { children, .. } = &mut nodes[idx] {
            insert_at_path(children, node, rest);
        }
    } else {
        let mut children = Vec::new();
        insert_at_path(&mut children, node, rest);
        nodes.push(RequestNode::Folder {
            id: Uuid::new_v4(),
            name: folder_name.to_owned(),
            children,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use birnio_core::{Auth, BasicAuth, BearerAuth};
    use serde_json::json;

    fn simple_request(name: &str, method: HttpMethod, url: &str) -> Request {
        Request {
            id: Uuid::new_v4(),
            name: name.to_string(),
            method,
            url: url.parse().unwrap(),
            headers: Vec::new(),
            body: Body::Empty,
            auth: Auth::None,
        }
    }

    fn collection(name: &str, nodes: Vec<RequestNode>) -> Collection {
        Collection {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
        }
    }

    #[test]
    fn roundtrip_empty_collection() {
        let col = collection("Empty", Vec::new());
        let restored = har_to_collection(collection_to_har(&col)).unwrap();
        assert_eq!(restored.id, col.id);
        assert_eq!(restored.name, col.name);
        assert!(restored.nodes.is_empty());
    }

    #[test]
    fn roundtrip_bearer_auth() {
        let mut req = simple_request("Profile", HttpMethod::Get, "https://api.example.com/me");
        req.auth = Auth::Bearer(BearerAuth {
            token: "tok123".to_string(),
        });

        let col = collection("Auth", vec![RequestNode::Request(req.clone())]);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();

        let RequestNode::Request(r) = &restored.nodes[0] else {
            panic!("expected request node");
        };
        assert_eq!(r.id, req.id);
        assert_eq!(r.name, req.name);
        assert_eq!(r.method, HttpMethod::Get);
        assert_eq!(r.url.as_str(), "https://api.example.com/me");
        assert_eq!(
            r.auth,
            Auth::Bearer(BearerAuth {
                token: "tok123".to_string()
            })
        );
    }

    #[test]
    fn roundtrip_basic_auth() {
        let mut req = simple_request("Login", HttpMethod::Post, "https://api.example.com/login");
        req.auth = Auth::Basic(BasicAuth {
            username: "user".to_string(),
            password: "pass".to_string(),
        });

        let col = collection("Auth", vec![RequestNode::Request(req.clone())]);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();

        let RequestNode::Request(r) = &restored.nodes[0] else {
            panic!()
        };
        assert_eq!(r.auth, req.auth);
    }

    #[test]
    fn roundtrip_empty_body() {
        let req = simple_request("Ping", HttpMethod::Head, "https://api.example.com/");
        let col = collection("Bodies", vec![RequestNode::Request(req)]);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();
        let RequestNode::Request(r) = &restored.nodes[0] else {
            panic!()
        };
        assert_eq!(r.body, Body::Empty);
    }

    #[test]
    fn roundtrip_json_body() {
        let mut req = simple_request(
            "Create User",
            HttpMethod::Post,
            "https://api.example.com/users",
        );
        req.body = Body::Json(json!({ "name": "Alice", "email": "alice@example.com" }));

        let col = collection("CRUD", vec![RequestNode::Request(req.clone())]);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();
        let RequestNode::Request(r) = &restored.nodes[0] else {
            panic!()
        };
        assert_eq!(r.body, req.body);
    }

    #[test]
    fn roundtrip_text_body() {
        let mut req = simple_request("Post Log", HttpMethod::Post, "https://api.example.com/log");
        req.body = Body::Text("hello world".to_string());

        let col = collection("Text", vec![RequestNode::Request(req.clone())]);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();
        let RequestNode::Request(r) = &restored.nodes[0] else {
            panic!()
        };
        assert_eq!(r.body, Body::Text("hello world".to_string()));
    }

    #[test]
    fn only_enabled_headers_in_har_field() {
        let mut req = simple_request("Headers", HttpMethod::Get, "https://api.example.com/");
        req.headers = vec![
            Header {
                name: "Accept".to_string(),
                value: "application/json".to_string(),
                enabled: true,
            },
            Header {
                name: "X-Debug".to_string(),
                value: "1".to_string(),
                enabled: false,
            },
        ];

        let har = collection_to_har(&collection("H", vec![RequestNode::Request(req)]));
        let entry = &har.log.entries[0];

        assert_eq!(entry.request.headers.len(), 1);
        assert_eq!(entry.request.headers[0].name, "Accept");
        assert_eq!(entry.birnio.headers.len(), 2);
    }

    #[test]
    fn roundtrip_headers_with_disabled() {
        let mut req = simple_request("Headers", HttpMethod::Get, "https://api.example.com/");
        req.headers = vec![
            Header {
                name: "Accept".to_string(),
                value: "application/json".to_string(),
                enabled: true,
            },
            Header {
                name: "X-Debug".to_string(),
                value: "1".to_string(),
                enabled: false,
            },
        ];

        let col = collection("H", vec![RequestNode::Request(req.clone())]);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();
        let RequestNode::Request(r) = &restored.nodes[0] else {
            panic!()
        };
        assert_eq!(r.headers.len(), 2);
        assert!(r.headers[0].enabled);
        assert!(!r.headers[1].enabled);
    }

    #[test]
    fn roundtrip_folder_structure() {
        let req1 = simple_request("List Pets", HttpMethod::Get, "https://api.example.com/pets");
        let req2 = simple_request(
            "Create Pet",
            HttpMethod::Post,
            "https://api.example.com/pets",
        );

        let col = collection(
            "Petstore",
            vec![RequestNode::Folder {
                id: Uuid::new_v4(),
                name: "Pets".to_string(),
                children: vec![
                    RequestNode::Request(req1.clone()),
                    RequestNode::Request(req2.clone()),
                ],
            }],
        );

        let har = collection_to_har(&col);
        assert_eq!(har.log.entries.len(), 2);
        assert_eq!(har.log.entries[0].birnio.folder, ["Pets"]);
        assert_eq!(har.log.entries[1].birnio.folder, ["Pets"]);

        let restored = har_to_collection(har).unwrap();
        assert_eq!(restored.nodes.len(), 1);
        let RequestNode::Folder { name, children, .. } = &restored.nodes[0] else {
            panic!()
        };
        assert_eq!(name, "Pets");
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn roundtrip_nested_folders() {
        let req = simple_request(
            "Get Dog",
            HttpMethod::Get,
            "https://api.example.com/pets/dogs/1",
        );
        let folder_id = Uuid::new_v4();
        let col = collection(
            "Zoo",
            vec![RequestNode::Folder {
                id: Uuid::new_v4(),
                name: "Pets".to_string(),
                children: vec![RequestNode::Folder {
                    id: folder_id,
                    name: "Dogs".to_string(),
                    children: vec![RequestNode::Request(req.clone())],
                }],
            }],
        );

        let har = collection_to_har(&col);
        assert_eq!(har.log.entries[0].birnio.folder, ["Pets", "Dogs"]);

        let restored = har_to_collection(har).unwrap();
        let RequestNode::Folder {
            children: pets_children,
            ..
        } = &restored.nodes[0]
        else {
            panic!()
        };
        let RequestNode::Folder {
            name: dogs_name,
            children: dogs_children,
            ..
        } = &pets_children[0]
        else {
            panic!()
        };
        assert_eq!(dogs_name, "Dogs");
        let RequestNode::Request(r) = &dogs_children[0] else {
            panic!()
        };
        assert_eq!(r.id, req.id);
    }

    #[test]
    fn invalid_method_returns_error() {
        let req = simple_request("Test", HttpMethod::Get, "https://api.example.com/");
        let col = collection("Test", vec![RequestNode::Request(req)]);
        let mut har = collection_to_har(&col);
        har.log.entries[0].request.method = "BREW".to_string();
        assert!(matches!(
            har_to_collection(har),
            Err(HarError::UnknownMethod(_))
        ));
    }

    #[test]
    fn multiple_requests_preserve_order() {
        let names = ["Alpha", "Beta", "Gamma", "Delta"];
        let nodes: Vec<RequestNode> = names
            .iter()
            .map(|n| {
                RequestNode::Request(simple_request(
                    n,
                    HttpMethod::Get,
                    "https://api.example.com/",
                ))
            })
            .collect();

        let col = collection("Order", nodes);
        let restored = har_to_collection(collection_to_har(&col)).unwrap();

        for (i, expected) in names.iter().enumerate() {
            let RequestNode::Request(r) = &restored.nodes[i] else {
                panic!()
            };
            assert_eq!(&r.name, expected);
        }
    }
}
