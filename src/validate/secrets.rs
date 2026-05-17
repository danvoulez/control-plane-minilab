use serde_json::Value;

const SECRET_FIELD_PATTERNS: &[&str] = &[
    "token",
    "api_key",
    "password",
    "secret",
    "service_role_key",
    "private_key",
    "bearer",
];

pub fn validate_no_secret_values_in_json(value: &Value) -> Result<(), String> {
    scan_value(value, "$")
}

fn scan_value(value: &Value, path: &str) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            let explicitly_redacted = map
                .get("redacted")
                .and_then(Value::as_bool)
                .unwrap_or(false)
                || map
                    .get("secret_redacted")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
            for (key, child) in map {
                let lower = key.to_ascii_lowercase();
                let secret_named = SECRET_FIELD_PATTERNS
                    .iter()
                    .any(|pattern| lower.contains(pattern));
                if secret_named && !safe_redacted_value(child, explicitly_redacted) {
                    return Err(format!(
                        "secret-like field `{path}.{key}` must not contain a value"
                    ));
                }
                scan_value(child, &format!("{path}.{key}"))?;
            }
            Ok(())
        }
        Value::Array(items) => {
            for (idx, child) in items.iter().enumerate() {
                scan_value(child, &format!("{path}[{idx}]"))?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn safe_redacted_value(value: &Value, parent_redacted: bool) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) if s.is_empty() => true,
        Value::String(s) if s == "[REDACTED]" || s == "***" || s == "<redacted>" => true,
        Value::Bool(false) => true,
        _ => parent_redacted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rejects_secret_named_fields_with_values() {
        for key in ["token", "api_key", "password", "secret", "service_role_key"] {
            let mut map = serde_json::Map::new();
            map.insert(key.to_string(), json!("printed-value"));
            assert!(validate_no_secret_values_in_json(&Value::Object(map)).is_err());
        }
    }
}
