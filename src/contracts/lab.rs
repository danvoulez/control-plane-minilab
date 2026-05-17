use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::EvidenceRef;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LabHostRef {
    pub host_id: String,
    pub display_name: Option<String>,
    pub runtime_entity_id: Option<String>,
    pub runtime_version: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabEvent {
    pub event_id: String,
    pub host_id: String,
    pub event_type: String,
    pub runtime_version: Option<String>,
    pub evidence: Option<EvidenceRef>,
    #[serde(default)]
    pub payload: Option<Value>,
    pub secret_redacted: bool,
    pub observed_at: String,
}
