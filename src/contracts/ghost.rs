use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GhostKind {
    MissingRegistryEntity,
    MissingConfig,
    ForbiddenConfig,
    MissingEvidence,
    RuntimeUnavailable,
    ReleaseUnpublished,
    AuthUnverified,
    ReceiptUnverified,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ghost {
    pub ghost_id: String,
    pub kind: GhostKind,
    pub subject: String,
    pub reason: String,
    pub detected_at: String,
}
