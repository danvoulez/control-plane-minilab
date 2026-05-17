use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecisionKind {
    Ok,
    Denied,
    NeedsApproval,
    Ghost,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyDecision {
    pub decision_id: String,
    pub action_id: String,
    pub engine: String,
    pub decision: PolicyDecisionKind,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default)]
    pub matched_policy_ids: Option<Vec<String>>,
    pub evaluated_at: String,
}
