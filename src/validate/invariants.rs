use serde_json::Value;

use crate::contracts::*;
use crate::validate::validate_no_secret_values_in_json;

pub fn validate_auth_context(ctx: &AuthContext) -> Result<(), String> {
    if ctx.token_value_printed {
        return Err("token_value_printed must be false".into());
    }
    if ctx.raw_token.as_deref().is_some_and(|raw| !raw.is_empty()) {
        return Err("raw token must not appear".into());
    }
    if ctx.verified
        && matches!(ctx.method, AuthMethod::None | AuthMethod::LocalDev)
        && !ctx.stubbed_verification
    {
        return Err("verified auth requires a real verifier or explicit stub".into());
    }
    Ok(())
}

pub fn derive_risk_class(verb: &str) -> RiskClass {
    let v = verb.to_ascii_lowercase();
    if contains_any(&v, &["execute", "run_command", "protected"]) {
        RiskClass::ProtectedPower
    } else if contains_any(&v, &["publish", "deploy", "provider_mutation"]) {
        RiskClass::ExternalEffect
    } else if contains_any(&v, &["delete", "drop", "remove"]) {
        RiskClass::Delete
    } else if v.contains("update") {
        RiskClass::Update
    } else if v.contains("install") {
        RiskClass::Install
    } else if contains_any(&v, &["register", "index", "project"]) {
        RiskClass::Write
    } else if contains_any(&v, &["doctor", "smoke", "health", "verify"]) {
        RiskClass::Diagnostic
    } else if contains_any(&v, &["list", "get", "status", "inspect", "read"]) {
        RiskClass::Read
    } else {
        RiskClass::ProtectedPower
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

pub fn validate_action_request(action: &ActionRequest) -> Result<(), String> {
    if action.actor.entity_id.trim().is_empty() {
        return Err("actor is required".into());
    }
    if action.resource.id.trim().is_empty() {
        return Err("resource is required".into());
    }
    let derived = derive_risk_class(&action.verb);
    if action.risk_class != derived {
        return Err(format!(
            "risk_class must be derived from verb: expected {derived:?}"
        ));
    }
    if matches!(
        action.risk_class,
        RiskClass::Install
            | RiskClass::Update
            | RiskClass::Delete
            | RiskClass::ExternalEffect
            | RiskClass::ProtectedPower
    ) && !action.dry_run
    {
        return Err("mutating/protected verbs cannot default to ok; gate/window required".into());
    }
    if let Some(metadata) = &action.metadata {
        validate_no_secret_values_in_json(metadata)?;
    }
    Ok(())
}

pub fn validate_policy_decision(decision: &PolicyDecision) -> Result<(), String> {
    if matches!(
        decision.decision,
        PolicyDecisionKind::Denied
            | PolicyDecisionKind::NeedsApproval
            | PolicyDecisionKind::Ghost
            | PolicyDecisionKind::Error
    ) && decision.reasons.is_empty()
    {
        return Err("reasons must not be empty for denied/needs_approval/ghost/error".into());
    }
    Ok(())
}

pub fn validate_gate_request(gate: &GateRequest) -> Result<(), String> {
    if gate.status != GateStatus::Pending {
        return Err("gate status must start as pending".into());
    }
    if gate.execution_window.is_some() {
        return Err("gate cannot create execution window by itself".into());
    }
    let reason = gate.reason.to_ascii_lowercase();
    if contains_any(
        &reason,
        &["protected", "update", "install", "delete", "publish"],
    ) && gate.required_approver.as_deref() != Some("dan")
    {
        return Err(
            "required_approver should be dan for protected/update/install/delete/publish".into(),
        );
    }
    Ok(())
}

pub fn validate_execution_window(window: &ExecutionWindow) -> Result<(), String> {
    if window.gate_id.trim().is_empty() {
        return Err("execution window must have gate_id".into());
    }
    if window.expires_at.trim().is_empty() {
        return Err("execution window must have expires_at".into());
    }
    if window.scope_hash.trim().is_empty() {
        return Err("execution window must have scope_hash".into());
    }
    if window.status == ExecutionWindowStatus::Consumed && window.consumed_at.is_none() {
        return Err("consumed window must have consumed_at".into());
    }
    Ok(())
}

pub fn validate_config_doctor_result(result: &ConfigDoctorResult) -> Result<(), String> {
    if result.secret_values_printed {
        return Err("secret_values_printed must be false".into());
    }
    if !result.keys_forbidden.is_empty() {
        return Err(format!(
            "forbidden config keys present: {}",
            result.keys_forbidden.join(", ")
        ));
    }
    for key in &result.keys_present {
        if key == "NEXT_PUBLIC_SUPABASE_SECRET_KEY"
            || key == "NEXT_PUBLIC_SUPABASE_SERVICE_ROLE_KEY"
        {
            return Err(format!("{key} is hard forbidden"));
        }
    }
    if result
        .legacy_keys_used
        .iter()
        .any(|k| k == "SUPABASE_SERVICE_ROLE_KEY")
        && result
            .canonical_keys_used
            .iter()
            .any(|k| k == "SUPABASE_SECRET_KEY")
    {
        return Err("SUPABASE_SECRET_KEY must be preferred over SUPABASE_SERVICE_ROLE_KEY".into());
    }
    Ok(())
}

pub fn validate_release_artifact_ref(artifact: &ReleaseArtifactRef) -> Result<(), String> {
    if artifact.storage_bucket.trim().is_empty() || artifact.storage_path.trim().is_empty() {
        return Err("storage_bucket and storage_path are required".into());
    }
    if artifact.status == ReleaseArtifactStatus::Published && artifact.sha256.trim().is_empty() {
        return Err("sha256 required for published artifacts".into());
    }
    if artifact.status != ReleaseArtifactStatus::Published {
        return Err("install/update requires status=published".into());
    }
    Ok(())
}

pub fn validate_lab_event(event: &LabEvent) -> Result<(), String> {
    if !event.secret_redacted {
        return Err("secret_redacted must be true".into());
    }
    if !matches!(event.host_id.as_str(), "lab8gb" | "lab256" | "lab512") {
        return Err("host_id must be lab8gb/lab256/lab512 for v0".into());
    }
    if let Some(payload) = &event.payload {
        validate_no_secret_values_in_json(payload)?;
    }
    if let Some(evidence) = &event.evidence {
        let evidence_value = serde_json::to_value(evidence).map_err(|err| err.to_string())?;
        validate_no_secret_values_in_json(&evidence_value)?;
    }
    let kind = event.event_type.to_ascii_lowercase();
    if contains_any(&kind, &["package", "install", "heartbeat"])
        && event.runtime_version.as_deref().unwrap_or("").is_empty()
    {
        return Err("runtime_version required for package/install/heartbeat when known".into());
    }
    Ok(())
}

pub fn validate_authority_response(response: &AuthorityResponse) -> Result<(), String> {
    if response.secret_values_printed {
        return Err("secret_values_printed must be false".into());
    }
    if response.ok && (!response.ghosts.is_empty() || !response.errors.is_empty()) {
        return Err("ok=true only when no ghosts/errors".into());
    }
    if response.ok && response.satisfied_conditions.len() < response.success_conditions.len() {
        return Err("ok=true requires satisfied success conditions".into());
    }
    if response.ok && response.status != AuthorityStatus::Ok {
        return Err("ok=true requires status=ok".into());
    }
    if response.status == AuthorityStatus::Ok && !response.ghosts.is_empty() {
        return Err("unknowns must become ghosts, not ok".into());
    }
    if let Some(data) = &response.data {
        validate_no_secret_values_in_json(data)?;
    }
    Ok(())
}

pub fn validate_contract_value(value: &Value) -> Result<(), String> {
    validate_no_secret_values_in_json(value)?;
    let name = value
        .get("contract_name")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            "contract_name missing; returning ghost/error instead of panic".to_string()
        })?;
    match name {
        "ServiceContract"
        | "CommandEnvelope"
        | "PrincipalRef"
        | "AuthContext"
        | "ActionRequest"
        | "ResourceRef"
        | "PolicyDecision"
        | "GateRequest"
        | "ExecutionWindow"
        | "ConfigDoctorResult"
        | "RegistryEntityRef"
        | "ReleaseArtifactRef"
        | "HostRuntimeUpdateRequest"
        | "LabEvent"
        | "EvidenceRef"
        | "ReceiptRef"
        | "Ghost"
        | "AuthorityResponse" => Ok(()),
        other => Err(format!("unknown contract_name: {other}")),
    }
}
