use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    Entity,
    Lab,
    Runtime,
    ReleaseArtifact,
    ConfigKey,
    Receipt,
    Database,
    StorageObject,
    McpTool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceRef {
    pub kind: ResourceKind,
    pub id: String,
    pub table: Option<String>,
    pub schema: Option<String>,
}

impl ResourceRef {
    pub fn new(kind: ResourceKind, id: impl Into<String>) -> Self {
        Self {
            kind,
            id: id.into(),
            table: None,
            schema: None,
        }
    }
}
