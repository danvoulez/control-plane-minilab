use serde::{Deserialize, Serialize};

use super::EvidenceMode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryEntityKind {
    Person,
    Computer,
    Service,
    Runtime,
    Database,
    Vault,
    App,
    Llm,
    Package,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryEntityStatus {
    Active,
    Planned,
    Partial,
    Ghost,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistryEntityRef {
    pub entity_id: String,
    pub name: Option<String>,
    pub kind: RegistryEntityKind,
    pub status: RegistryEntityStatus,
    pub evidence_status: EvidenceMode,
}
