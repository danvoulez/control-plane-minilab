use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseArtifactStatus {
    Planned,
    Built,
    Published,
    Retired,
    Ghost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseArtifactRef {
    pub artifact_id: String,
    pub package_name: String,
    pub version: String,
    pub storage_bucket: String,
    pub storage_path: String,
    pub sha256: String,
    pub size_bytes: Option<u64>,
    pub status: ReleaseArtifactStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostRuntimeUpdateRequest {
    pub request_id: String,
    pub host_id: String,
    pub current_version: Option<String>,
    pub target_artifact: ReleaseArtifactRef,
    pub doctor_required: bool,
    pub smoke_required: bool,
    pub requested_at: String,
    pub dry_run: bool,
}
