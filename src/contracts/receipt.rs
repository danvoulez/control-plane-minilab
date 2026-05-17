use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptStatus {
    Open,
    Closed,
    Failed,
    Ghost,
    Unverified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiptRef {
    pub receipt_id: String,
    pub status: ReceiptStatus,
    pub evidence_ids: Vec<String>,
    pub closed_at: Option<String>,
    pub verifier: Option<String>,
}
