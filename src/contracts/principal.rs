use serde::{Deserialize, Serialize};

use super::AuthContext;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalKind {
    Human,
    Llm,
    Service,
    Lab,
    Runtime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrincipalRef {
    pub entity_id: String,
    pub kind: PrincipalKind,
    pub display_name: Option<String>,
    pub auth_context: Option<AuthContext>,
}

impl PrincipalRef {
    pub fn new(entity_id: impl Into<String>, kind: PrincipalKind) -> Self {
        Self {
            entity_id: entity_id.into(),
            kind,
            display_name: None,
            auth_context: None,
        }
    }
}
