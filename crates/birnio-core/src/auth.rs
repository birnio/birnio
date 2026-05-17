use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Auth {
    #[default]
    None,
    Basic(BasicAuth),
    Bearer(BearerAuth),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BearerAuth {
    pub token: String,
}
