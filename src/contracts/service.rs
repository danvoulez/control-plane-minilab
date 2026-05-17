use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceContract {
    pub contract_name: String,
    pub service_id: String,
    pub version: String,
    pub role: String,
    pub mode: String,
    pub external_effects_enabled: bool,
    pub started_at: String,
}

impl ServiceContract {
    pub fn cli() -> Self {
        Self {
            contract_name: "ServiceContract".to_string(),
            service_id: "minilab-control-plane-authority".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            role: "control_plane_authority_core".to_string(),
            mode: "cli".to_string(),
            external_effects_enabled: false,
            started_at: crate::cli::now_string(),
        }
    }
}
