use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::PrincipalRef;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandEnvelope {
    pub command_id: String,
    pub command: String,
    pub actor: PrincipalRef,
    pub input: Value,
    pub requested_at: String,
    pub correlation_id: Option<String>,
    pub dry_run: bool,
}
