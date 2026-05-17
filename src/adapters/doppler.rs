use std::env;

use crate::{adapters::AuthorityError, contracts::AuthorityStatus};

const FORBIDDEN_ENV_KEYS: &[&str] = &[
    "NEXT_PUBLIC_SUPABASE_SECRET_KEY",
    "NEXT_PUBLIC_SUPABASE_SERVICE_ROLE_KEY",
];

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct DopplerEnvInspection {
    pub status: AuthorityStatus,
    pub doppler_project_present: bool,
    pub doppler_config_present: bool,
    pub doppler_environment_present: bool,
    pub warnings: Vec<String>,
    pub forbidden_keys: Vec<String>,
    pub secret_values_printed: bool,
}

#[derive(Debug, Default, Clone)]
pub struct DopplerEnvInspector;

impl DopplerEnvInspector {
    pub fn inspect_current_env(&self) -> Result<DopplerEnvInspection, AuthorityError> {
        self.inspect_keys(env::vars().map(|(key, _)| key))
    }

    pub fn inspect_keys<I, S>(&self, keys: I) -> Result<DopplerEnvInspection, AuthorityError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let keys: Vec<String> = keys
            .into_iter()
            .map(|key| key.as_ref().to_string())
            .collect();
        let forbidden_keys: Vec<String> = FORBIDDEN_ENV_KEYS
            .iter()
            .filter(|forbidden| keys.iter().any(|key| key == **forbidden))
            .map(|key| key.to_string())
            .collect();
        if !forbidden_keys.is_empty() {
            return Err(AuthorityError::ForbiddenConfig);
        }
        let doppler_project_present = contains(&keys, "DOPPLER_PROJECT");
        let doppler_config_present = contains(&keys, "DOPPLER_CONFIG");
        let doppler_environment_present = contains(&keys, "DOPPLER_ENVIRONMENT");
        let mut warnings = Vec::new();
        if !doppler_project_present {
            warnings.push("DOPPLER_PROJECT metadata missing".to_string());
        }
        if !doppler_config_present {
            warnings.push("DOPPLER_CONFIG metadata missing".to_string());
        }
        if !doppler_environment_present {
            warnings.push("DOPPLER_ENVIRONMENT metadata missing".to_string());
        }
        Ok(DopplerEnvInspection {
            status: if warnings.is_empty() {
                AuthorityStatus::Ok
            } else {
                AuthorityStatus::Warn
            },
            doppler_project_present,
            doppler_config_present,
            doppler_environment_present,
            warnings,
            forbidden_keys,
            secret_values_printed: false,
        })
    }
}

fn contains(keys: &[String], needle: &str) -> bool {
    keys.iter().any(|key| key == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_next_public_supabase_secret_key_returns_structured_error_no_panic() {
        let inspector = DopplerEnvInspector;
        assert_eq!(
            inspector
                .inspect_keys(["NEXT_PUBLIC_SUPABASE_SECRET_KEY"])
                .unwrap_err(),
            AuthorityError::ForbiddenConfig
        );
    }

    #[test]
    fn missing_metadata_warns_not_crashes() {
        let inspector = DopplerEnvInspector;
        let result = inspector.inspect_keys(["SUPABASE_URL"]).unwrap();
        assert_eq!(result.status, AuthorityStatus::Warn);
        assert!(!result.secret_values_printed);
    }
}
