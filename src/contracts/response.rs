use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Ghost;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityStatus {
    Ok,
    Warn,
    Error,
    Ghost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorityResponse {
    pub ok: bool,
    pub status: AuthorityStatus,
    pub message: String,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub ghosts: Vec<Ghost>,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default)]
    pub success_conditions: Vec<String>,
    #[serde(default)]
    pub satisfied_conditions: Vec<String>,
    pub secret_values_printed: bool,
}
