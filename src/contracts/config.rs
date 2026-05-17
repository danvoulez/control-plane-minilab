use serde::{Deserialize, Serialize};

use super::AuthorityStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigDoctorResult {
    pub run_id: String,
    pub status: AuthorityStatus,
    pub doppler_project: Option<String>,
    pub doppler_config: Option<String>,
    #[serde(default)]
    pub keys_present: Vec<String>,
    #[serde(default)]
    pub keys_missing: Vec<String>,
    #[serde(default)]
    pub keys_unknown: Vec<String>,
    #[serde(default)]
    pub keys_forbidden: Vec<String>,
    #[serde(default)]
    pub canonical_keys_used: Vec<String>,
    #[serde(default)]
    pub legacy_keys_used: Vec<String>,
    pub secret_values_printed: bool,
    pub checked_at: String,
}
