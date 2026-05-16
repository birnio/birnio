use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::Header;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
    pub elapsed: Duration,
}
