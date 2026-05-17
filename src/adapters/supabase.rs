use std::{env, time::Duration};

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

use crate::contracts::*;

const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AuthorityError {
    #[error("database_unavailable")]
    DatabaseUnavailable,
    #[error("database_auth_unconfigured")]
    DatabaseAuthUnconfigured,
    #[error("database_auth_failed")]
    DatabaseAuthFailed,
    #[error("database_schema_unavailable")]
    DatabaseSchemaUnavailable,
    #[error("adapter_contract_mismatch")]
    AdapterContractMismatch,
    #[error("missing_registry_entity")]
    MissingRegistryEntity,
    #[error("missing_release_artifact")]
    MissingReleaseArtifact,
    #[error("missing_config_key")]
    MissingConfigKey,
    #[error("release_unpublished")]
    ReleaseUnpublished,
    #[error("forbidden_config")]
    ForbiddenConfig,
}

impl AuthorityError {
    pub fn code(&self) -> &'static str {
        match self {
            AuthorityError::DatabaseUnavailable => "database_unavailable",
            AuthorityError::DatabaseAuthUnconfigured => "database_auth_unconfigured",
            AuthorityError::DatabaseAuthFailed => "database_auth_failed",
            AuthorityError::DatabaseSchemaUnavailable => "database_schema_unavailable",
            AuthorityError::AdapterContractMismatch => "adapter_contract_mismatch",
            AuthorityError::MissingRegistryEntity => "missing_registry_entity",
            AuthorityError::MissingReleaseArtifact => "missing_release_artifact",
            AuthorityError::MissingConfigKey => "missing_config_key",
            AuthorityError::ReleaseUnpublished => "release_unpublished",
            AuthorityError::ForbiddenConfig => "forbidden_config",
        }
    }
}

pub trait SupabaseReadAdapter {
    fn get_entity(&self, entity_id: &str) -> Result<RegistryEntityRef, AuthorityError>;
    fn get_lab_host(&self, host_id: &str) -> Result<LabHostRef, AuthorityError>;
    fn get_release_artifact(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<ReleaseArtifactRef, AuthorityError>;
    fn get_latest_published_artifact(
        &self,
        package_name: &str,
    ) -> Result<ReleaseArtifactRef, AuthorityError>;
    fn get_config_key(&self, key_name: &str) -> Result<ConfigKeyRef, AuthorityError>;
    fn get_runtime_config_requirements(
        &self,
        runtime_entity_id: &str,
    ) -> Result<Vec<String>, AuthorityError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SupabaseCredential {
    SecretKey(String),
    LegacyServiceRole(String),
}

#[derive(Debug, Clone)]
pub struct SupabaseHttpReadAdapter {
    base_url: String,
    credential: SupabaseCredential,
    client: Client,
}

impl SupabaseHttpReadAdapter {
    pub fn from_env() -> Result<Self, AuthorityError> {
        let base_url = env::var("SUPABASE_URL").map_err(|_| AuthorityError::DatabaseUnavailable)?;
        let credential = if let Ok(secret) = env::var("SUPABASE_SECRET_KEY") {
            SupabaseCredential::SecretKey(secret)
        } else if let Ok(legacy) = env::var("SUPABASE_SERVICE_ROLE_KEY") {
            eprintln!("warning: SUPABASE_SERVICE_ROLE_KEY legacy fallback in use; prefer SUPABASE_SECRET_KEY");
            SupabaseCredential::LegacyServiceRole(legacy)
        } else {
            return Err(AuthorityError::DatabaseAuthUnconfigured);
        };
        Self::new(base_url, credential)
    }

    #[cfg(test)]
    pub fn with_secret(
        base_url: impl Into<String>,
        secret: impl Into<String>,
    ) -> Result<Self, AuthorityError> {
        Self::new(
            base_url.into(),
            SupabaseCredential::SecretKey(secret.into()),
        )
    }

    #[cfg(test)]
    pub fn with_legacy(
        base_url: impl Into<String>,
        secret: impl Into<String>,
    ) -> Result<Self, AuthorityError> {
        Self::new(
            base_url.into(),
            SupabaseCredential::LegacyServiceRole(secret.into()),
        )
    }

    fn new(base_url: String, credential: SupabaseCredential) -> Result<Self, AuthorityError> {
        let client = Client::builder()
            .timeout(TIMEOUT)
            .no_proxy()
            .build()
            .map_err(|_| AuthorityError::DatabaseUnavailable)?;
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            credential,
            client,
        })
    }

    fn get_rows<T: DeserializeOwned>(
        &self,
        path: &str,
        schema: &str,
        query: &[(&str, String)],
    ) -> Result<Vec<T>, AuthorityError> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self
            .client
            .get(url)
            .header("apikey", self.secret_value())
            .header("Accept", "application/json")
            .header("Accept-Profile", schema);
        if let SupabaseCredential::LegacyServiceRole(secret) = &self.credential {
            request = request.header("Authorization", format!("Bearer {secret}"));
        }
        for (key, value) in query {
            request = request.query(&[(key, value)]);
        }
        let response = request.send().map_err(|err| {
            if err.is_timeout() {
                AuthorityError::DatabaseUnavailable
            } else {
                AuthorityError::DatabaseUnavailable
            }
        })?;
        let status = response.status().as_u16();
        let body = response
            .text()
            .map_err(|_| AuthorityError::DatabaseUnavailable)?;
        if status == 401 || status == 403 {
            return Err(AuthorityError::DatabaseAuthFailed);
        }
        if status == 404 || contains_pgrst106(&body) {
            return Err(AuthorityError::DatabaseSchemaUnavailable);
        }
        if !(200..300).contains(&status) {
            return Err(AuthorityError::DatabaseUnavailable);
        }
        serde_json::from_str(&body).map_err(|_| AuthorityError::AdapterContractMismatch)
    }

    fn secret_value(&self) -> &str {
        match &self.credential {
            SupabaseCredential::SecretKey(secret)
            | SupabaseCredential::LegacyServiceRole(secret) => secret,
        }
    }
}

impl SupabaseReadAdapter for SupabaseHttpReadAdapter {
    fn get_entity(&self, entity_id: &str) -> Result<RegistryEntityRef, AuthorityError> {
        one(
            self.get_rows(
                "/rest/v1/entities",
                "registry",
                &[("entity_id", eq(entity_id)), ("limit", "1".into())],
            )?,
            AuthorityError::MissingRegistryEntity,
        )
    }

    fn get_lab_host(&self, host_id: &str) -> Result<LabHostRef, AuthorityError> {
        one(
            self.get_rows(
                "/rest/v1/hosts",
                "lab_observability",
                &[("host_id", eq(host_id)), ("limit", "1".into())],
            )?,
            AuthorityError::MissingRegistryEntity,
        )
    }

    fn get_release_artifact(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<ReleaseArtifactRef, AuthorityError> {
        let artifact = one(
            self.get_rows(
                "/rest/v1/artifacts",
                "release_registry",
                &[
                    ("package_name", eq(package_name)),
                    ("version", eq(version)),
                    ("limit", "1".into()),
                ],
            )?,
            AuthorityError::MissingReleaseArtifact,
        )?;
        ensure_published(artifact)
    }

    fn get_latest_published_artifact(
        &self,
        package_name: &str,
    ) -> Result<ReleaseArtifactRef, AuthorityError> {
        one(
            self.get_rows(
                "/rest/v1/artifacts",
                "release_registry",
                &[
                    ("package_name", eq(package_name)),
                    ("status", eq("published")),
                    ("order", "version.desc".into()),
                    ("limit", "1".into()),
                ],
            )?,
            AuthorityError::MissingReleaseArtifact,
        )
    }

    fn get_config_key(&self, key_name: &str) -> Result<ConfigKeyRef, AuthorityError> {
        one(
            self.get_rows(
                "/rest/v1/keys",
                "config_registry",
                &[("key_name", eq(key_name)), ("limit", "1".into())],
            )?,
            AuthorityError::MissingConfigKey,
        )
    }

    fn get_runtime_config_requirements(
        &self,
        runtime_entity_id: &str,
    ) -> Result<Vec<String>, AuthorityError> {
        #[derive(serde::Deserialize)]
        struct RequirementRow {
            key_name: String,
        }
        let rows: Vec<RequirementRow> = self.get_rows(
            "/rest/v1/config_requirements",
            "registry",
            &[("runtime_entity_id", eq(runtime_entity_id))],
        )?;
        Ok(rows.into_iter().map(|row| row.key_name).collect())
    }
}

fn one<T>(rows: Vec<T>, missing: AuthorityError) -> Result<T, AuthorityError> {
    rows.into_iter().next().ok_or(missing)
}

fn eq(value: &str) -> String {
    format!("eq.{value}")
}

fn ensure_published(artifact: ReleaseArtifactRef) -> Result<ReleaseArtifactRef, AuthorityError> {
    if artifact.status == ReleaseArtifactStatus::Published {
        Ok(artifact)
    } else {
        Err(AuthorityError::ReleaseUnpublished)
    }
}

fn contains_pgrst106(body: &str) -> bool {
    serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("code")
                .and_then(Value::as_str)
                .map(|s| s == "PGRST106")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::{Mutex, OnceLock},
        thread,
    };

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn serve(status: u16, body: &'static str) -> (String, thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0; 8192];
            let n = stream.read(&mut buf).unwrap();
            let request = String::from_utf8_lossy(&buf[..n]).to_string();
            write!(stream, "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", body.len(), body).unwrap();
            request
        });
        (format!("http://{addr}"), handle)
    }

    fn fixture(name: &str) -> String {
        std::fs::read_to_string(format!("tests/fixtures/supabase/{name}")).unwrap()
    }

    #[test]
    fn missing_supabase_url_returns_database_unavailable() {
        let _guard = env_lock();
        env::remove_var("SUPABASE_URL");
        env::remove_var("SUPABASE_SECRET_KEY");
        env::remove_var("SUPABASE_SERVICE_ROLE_KEY");
        assert_eq!(
            SupabaseHttpReadAdapter::from_env().unwrap_err(),
            AuthorityError::DatabaseUnavailable
        );
    }

    #[test]
    fn missing_secret_returns_database_auth_unconfigured() {
        let _guard = env_lock();
        env::set_var("SUPABASE_URL", "http://127.0.0.1:1");
        env::remove_var("SUPABASE_SECRET_KEY");
        env::remove_var("SUPABASE_SERVICE_ROLE_KEY");
        assert_eq!(
            SupabaseHttpReadAdapter::from_env().unwrap_err(),
            AuthorityError::DatabaseAuthUnconfigured
        );
    }

    #[test]
    fn supabase_secret_key_preferred_over_service_role_key() {
        let _guard = env_lock();
        env::set_var("SUPABASE_URL", "http://127.0.0.1:1");
        env::set_var("SUPABASE_SECRET_KEY", "sb_secret_new");
        env::set_var("SUPABASE_SERVICE_ROLE_KEY", "legacy");
        let adapter = SupabaseHttpReadAdapter::from_env().unwrap();
        assert!(matches!(
            adapter.credential,
            SupabaseCredential::SecretKey(_)
        ));
    }

    #[test]
    fn sb_secret_sends_apikey_only_no_authorization() {
        let body = Box::leak(format!("[{}]", fixture("registry_entity_dan.json")).into_boxed_str());
        let (url, handle) = serve(200, body);
        let adapter = SupabaseHttpReadAdapter::with_secret(url, "sb_secret_test").unwrap();
        adapter.get_entity("dan").unwrap();
        let request = handle.join().unwrap();
        let lower = request.to_ascii_lowercase();
        assert!(lower.contains("apikey: sb_secret_test"));
        assert!(lower.contains("accept-profile: registry"));
        assert!(!lower.contains("authorization:"));
    }

    #[test]
    fn legacy_sends_apikey_and_authorization() {
        let body = Box::leak(format!("[{}]", fixture("registry_entity_dan.json")).into_boxed_str());
        let (url, handle) = serve(200, body);
        let adapter = SupabaseHttpReadAdapter::with_legacy(url, "legacy.jwt").unwrap();
        adapter.get_entity("dan").unwrap();
        let request = handle.join().unwrap();
        let lower = request.to_ascii_lowercase();
        assert!(lower.contains("apikey: legacy.jwt"));
        assert!(lower.contains("authorization: bearer legacy.jwt"));
    }

    #[test]
    fn planned_release_is_not_installable() {
        let body =
            Box::leak(format!("[{}]", fixture("release_artifact_planned.json")).into_boxed_str());
        let (url, _) = serve(200, body);
        let adapter = SupabaseHttpReadAdapter::with_secret(url, "sb_secret_test").unwrap();
        assert_eq!(
            adapter
                .get_release_artifact("minilab-host-runtime", "0.2.0")
                .unwrap_err(),
            AuthorityError::ReleaseUnpublished
        );
    }

    #[test]
    fn published_release_is_resolvable() {
        let body =
            Box::leak(format!("[{}]", fixture("release_artifact_published.json")).into_boxed_str());
        let (url, _) = serve(200, body);
        let adapter = SupabaseHttpReadAdapter::with_secret(url, "sb_secret_test").unwrap();
        assert_eq!(
            adapter
                .get_latest_published_artifact("minilab-host-runtime")
                .unwrap()
                .status,
            ReleaseArtifactStatus::Published
        );
    }

    #[test]
    fn auth_failure_maps() {
        let (url, _) = serve(401, "{}");
        let adapter = SupabaseHttpReadAdapter::with_secret(url, "sb_secret_test").unwrap();
        assert_eq!(
            adapter.get_entity("dan").unwrap_err(),
            AuthorityError::DatabaseAuthFailed
        );
    }

    #[test]
    fn pgrst106_maps_schema_unavailable() {
        let (url, _) = serve(404, r#"{"code":"PGRST106"}"#);
        let adapter = SupabaseHttpReadAdapter::with_secret(url, "sb_secret_test").unwrap();
        assert_eq!(
            adapter.get_entity("dan").unwrap_err(),
            AuthorityError::DatabaseSchemaUnavailable
        );
    }

    #[test]
    fn malformed_json_maps_contract_mismatch() {
        let (url, _) = serve(200, "not-json");
        let adapter = SupabaseHttpReadAdapter::with_secret(url, "sb_secret_test").unwrap();
        assert_eq!(
            adapter.get_entity("dan").unwrap_err(),
            AuthorityError::AdapterContractMismatch
        );
    }
}
