use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    None,
    BearerToken,
    Jwt,
    Oidc,
    Passkey,
    McpToken,
    LocalDev,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthContext {
    pub method: AuthMethod,
    pub verified: bool,
    pub subject: Option<String>,
    pub issuer: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
    pub token_present: bool,
    pub token_value_printed: bool,
    #[serde(default)]
    pub stubbed_verification: bool,
    #[serde(default)]
    pub raw_token: Option<String>,
}
