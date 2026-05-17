use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceMode {
    Declared,
    Observed,
    Verified,
    Unverified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceRef {
    pub evidence_id: String,
    pub mode: EvidenceMode,
    pub source: String,
    pub observed_at: String,
    pub summary: String,
    pub secret_redacted: bool,
}
