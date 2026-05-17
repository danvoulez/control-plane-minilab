use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{PrincipalRef, ResourceRef};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskClass {
    Read,
    Diagnostic,
    Write,
    Install,
    Update,
    Delete,
    ExternalEffect,
    ProtectedPower,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionRequest {
    pub action_id: String,
    pub actor: PrincipalRef,
    pub verb: String,
    pub resource: ResourceRef,
    pub intent: Option<String>,
    pub risk_class: RiskClass,
    pub requested_at: String,
    pub dry_run: bool,
    #[serde(default)]
    pub metadata: Option<Value>,
}
