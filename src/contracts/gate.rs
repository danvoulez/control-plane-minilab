use serde::{Deserialize, Serialize};

use super::{PrincipalRef, ResourceRef};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Pending,
    Approved,
    Denied,
    Ghosted,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GateRequest {
    pub gate_id: String,
    pub action_id: String,
    pub actor: PrincipalRef,
    pub resource: ResourceRef,
    pub policy_decision_id: String,
    pub status: GateStatus,
    pub requested_at: String,
    pub expires_at: Option<String>,
    pub required_approver: Option<String>,
    pub reason: String,
    #[serde(default)]
    pub execution_window: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionWindowStatus {
    Open,
    Consumed,
    Expired,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionWindow {
    pub window_id: String,
    pub gate_id: String,
    pub actor_entity_id: String,
    pub allowed_action: String,
    pub allowed_resource_id: String,
    pub scope_hash: String,
    pub expires_at: String,
    pub consumed_at: Option<String>,
    pub status: ExecutionWindowStatus,
}
