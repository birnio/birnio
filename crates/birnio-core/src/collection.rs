use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Request;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub nodes: Vec<RequestNode>,
}

impl Collection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            nodes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RequestNode {
    Folder {
        id: Uuid,
        name: String,
        children: Vec<RequestNode>,
    },
    Request(Request),
}
