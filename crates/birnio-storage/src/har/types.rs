use birnio_core::Auth;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Root HAR document wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarDocument {
    pub log: HarLog,
}

/// HAR log — one per collection `.har` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarLog {
    pub version: String,
    pub creator: HarCreator,
    pub entries: Vec<HarEntry>,
    #[serde(rename = "_birnio")]
    pub birnio: BirnioCollection,
}

/// HAR creator metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarCreator {
    pub name: String,
    pub version: String,
}

/// A single HAR entry representing one request/response pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarEntry {
    pub request: HarRequest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HarResponse>,
    #[serde(rename = "_birnio")]
    pub birnio: BirnioRequest,
}

/// HAR HTTP request record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarRequest {
    pub method: String,
    pub url: String,
    #[serde(rename = "httpVersion")]
    pub http_version: String,
    pub cookies: Vec<HarNameValue>,
    pub headers: Vec<HarNameValue>,
    #[serde(rename = "queryString")]
    pub query_string: Vec<HarNameValue>,
    #[serde(rename = "postData", skip_serializing_if = "Option::is_none")]
    pub post_data: Option<HarPostData>,
    #[serde(rename = "headersSize")]
    pub headers_size: i64,
    #[serde(rename = "bodySize")]
    pub body_size: i64,
}

/// HAR HTTP response record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarResponse {
    pub status: u16,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "httpVersion")]
    pub http_version: String,
    pub cookies: Vec<HarNameValue>,
    pub headers: Vec<HarNameValue>,
    pub content: HarContent,
    #[serde(rename = "redirectURL")]
    pub redirect_url: String,
    #[serde(rename = "headersSize")]
    pub headers_size: i64,
    #[serde(rename = "bodySize")]
    pub body_size: i64,
}

/// HAR response content body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarContent {
    pub size: i64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// A generic name/value pair used for headers, query strings, and cookies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarNameValue {
    pub name: String,
    pub value: String,
}

/// HAR request body payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarPostData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
}

// ── Birnio extension fields ─────────────────────────────────────────────────

/// Birnio collection metadata stored in `log._birnio`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirnioCollection {
    pub id: Uuid,
    pub name: String,
}

/// Birnio request metadata stored in `entry._birnio`.
///
/// Stores Birnio-specific data with no equivalent in standard HAR, including
/// authentication config and the full header list with `enabled` flags for
/// round-trip fidelity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirnioRequest {
    pub id: Uuid,
    pub name: String,
    pub auth: Auth,
    /// Complete header list including disabled entries — enables round-trip of
    /// headers that are toggled off. The standard `request.headers` field
    /// contains only enabled headers for HAR compatibility.
    pub headers: Vec<BirnioHeader>,
    /// Folder path from the collection root, e.g. `["Pets", "CRUD"]`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folder: Vec<String>,
}

/// A header entry as stored in `_birnio`, preserving the `enabled` flag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirnioHeader {
    pub name: String,
    pub value: String,
    pub enabled: bool,
}
