use std::{
    env, fs,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use serde_json::{json, Value};

use crate::{contracts::*, output::print_json, validate::*};

const CONTRACT_NAMES: &[&str] = &[
    "ServiceContract",
    "CommandEnvelope",
    "PrincipalRef",
    "AuthContext",
    "ActionRequest",
    "ResourceRef",
    "PolicyDecision",
    "GateRequest",
    "ExecutionWindow",
    "ConfigDoctorResult",
    "RegistryEntityRef",
    "ReleaseArtifactRef",
    "HostRuntimeUpdateRequest",
    "LabEvent",
    "EvidenceRef",
    "ReceiptRef",
    "Ghost",
    "AuthorityResponse",
];

#[derive(Debug, Parser)]
#[command(
    name = "minilab-control-plane-authority",
    version,
    about = "Control Plane Authority Core contracts CLI"
)]
pub struct Cli {
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Status,
    Contracts {
        #[command(subcommand)]
        command: ContractsCommand,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    Gate {
        #[command(subcommand)]
        command: GateCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ContractsCommand {
    List,
    Check(CheckArgs),
}

#[derive(Debug, Args)]
struct CheckArgs {
    #[arg(long)]
    file: String,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Doctor(DoctorArgs),
}

#[derive(Debug, Args)]
struct DoctorArgs {
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Subcommand)]
enum PolicyCommand {
    Check(ActionArgs),
}

#[derive(Debug, Subcommand)]
enum GateCommand {
    Decide(ActionArgs),
}

#[derive(Debug, Args, Clone)]
pub struct ActionArgs {
    #[arg(long)]
    actor: String,
    #[arg(long)]
    action: String,
    #[arg(long)]
    resource: String,
    #[arg(long)]
    json: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Status => status(cli.json),
        Commands::Contracts { command } => match command {
            ContractsCommand::List => contracts_list(cli.json),
            ContractsCommand::Check(args) => contracts_check(&args.file, cli.json),
        },
        Commands::Config { command } => match command {
            ConfigCommand::Doctor(args) => config_doctor(args.dry_run, cli.json),
        },
        Commands::Policy { command } => match command {
            PolicyCommand::Check(args) => policy_check(args.clone(), cli.json || args.json),
        },
        Commands::Gate { command } => match command {
            GateCommand::Decide(args) => gate_decide(args.clone(), cli.json || args.json),
        },
    }
}

fn status(as_json: bool) -> Result<()> {
    let service = ServiceContract::cli();
    if as_json {
        print_json(&service)
    } else {
        println!(
            "{} {} ({}) external_effects_enabled={}",
            service.service_id, service.version, service.role, service.external_effects_enabled
        );
        Ok(())
    }
}

fn contracts_list(as_json: bool) -> Result<()> {
    if as_json {
        print_json(&json!({ "contracts": CONTRACT_NAMES }))
    } else {
        for name in CONTRACT_NAMES {
            println!("{name}");
        }
        Ok(())
    }
}

fn contracts_check(file: &str, as_json: bool) -> Result<()> {
    let contents = fs::read_to_string(file)?;
    let value: Value = serde_json::from_str(&contents)?;
    let result = match validate_contract_value(&value) {
        Ok(()) => {
            json!({ "ok": true, "status": "ok", "file": file, "secret_values_printed": false })
        }
        Err(err) => json!({
            "ok": false,
            "status": if err.contains("contract_name missing") { "ghost" } else { "error" },
            "file": file,
            "error": err,
            "secret_values_printed": false
        }),
    };
    if as_json {
        print_json(&result)
    } else {
        println!("{}", serde_json::to_string_pretty(&result)?);
        Ok(())
    }
}

fn config_doctor(dry_run: bool, as_json: bool) -> Result<()> {
    if !dry_run {
        return Err(anyhow!(
            "config doctor v0 only supports --dry-run; no Doppler calls are made"
        ));
    }
    let result = config_doctor_from_env(env::vars().map(|(k, _)| k).collect());
    if as_json {
        print_json(&result)
    } else {
        println!("config doctor status={:?}", result.status);
        println!("keys_present={}", result.keys_present.join(","));
        println!("keys_missing={}", result.keys_missing.join(","));
        println!("keys_forbidden={}", result.keys_forbidden.join(","));
        Ok(())
    }
}

pub fn config_doctor_from_env(mut keys: Vec<String>) -> ConfigDoctorResult {
    keys.sort();
    let required = [
        "SUPABASE_URL",
        "SUPABASE_SECRET_KEY",
        "SUPABASE_PUBLISHABLE_KEY",
    ];
    let forbidden = [
        "NEXT_PUBLIC_SUPABASE_SECRET_KEY",
        "NEXT_PUBLIC_SUPABASE_SERVICE_ROLE_KEY",
    ];
    let keys_forbidden: Vec<String> = forbidden
        .iter()
        .filter(|k| keys.iter().any(|present| present == **k))
        .map(|s| s.to_string())
        .collect();
    let mut canonical_keys_used = Vec::new();
    let mut legacy_keys_used = Vec::new();
    if keys.iter().any(|k| k == "SUPABASE_SECRET_KEY") {
        canonical_keys_used.push("SUPABASE_SECRET_KEY".to_string());
    } else if keys.iter().any(|k| k == "SUPABASE_SERVICE_ROLE_KEY") {
        legacy_keys_used.push("SUPABASE_SERVICE_ROLE_KEY".to_string());
    }
    if keys.iter().any(|k| k == "SUPABASE_PUBLISHABLE_KEY") {
        canonical_keys_used.push("SUPABASE_PUBLISHABLE_KEY".to_string());
    } else if keys.iter().any(|k| k == "SUPABASE_ANON_KEY") {
        legacy_keys_used.push("SUPABASE_ANON_KEY".to_string());
    }
    let keys_missing: Vec<String> = required
        .iter()
        .filter(|key| !keys.iter().any(|present| present == **key))
        .map(|s| s.to_string())
        .collect();
    let status = if !keys_forbidden.is_empty() {
        AuthorityStatus::Error
    } else if !keys_missing.is_empty() || !legacy_keys_used.is_empty() {
        AuthorityStatus::Warn
    } else {
        AuthorityStatus::Ok
    };
    ConfigDoctorResult {
        run_id: "doctor_local_env".to_string(),
        status,
        doppler_project: env::var("DOPPLER_PROJECT").ok(),
        doppler_config: env::var("DOPPLER_CONFIG").ok(),
        keys_present: keys,
        keys_missing,
        keys_unknown: Vec::new(),
        keys_forbidden,
        canonical_keys_used,
        legacy_keys_used,
        secret_values_printed: false,
        checked_at: now_string(),
    }
}

fn policy_check(args: ActionArgs, as_json: bool) -> Result<()> {
    let decision = stub_policy_v0(&args.actor, &args.action, &args.resource);
    if as_json {
        print_json(&decision)
    } else {
        println!(
            "policy decision={:?} reasons={}",
            decision.decision,
            decision.reasons.join("; ")
        );
        Ok(())
    }
}

pub fn stub_policy_v0(actor: &str, action: &str, resource: &str) -> PolicyDecision {
    let mut reasons = Vec::new();
    let decision =
        if actor.trim().is_empty() || action.trim().is_empty() || resource.trim().is_empty() {
            reasons.push("unknown or empty actor/resource/action becomes ghost/error".to_string());
            PolicyDecisionKind::Ghost
        } else if is_llm_actor(actor)
            && matches!(derive_risk_class(action), RiskClass::ProtectedPower)
        {
            reasons.push("LLM cannot execute protected power".to_string());
            PolicyDecisionKind::Denied
        } else if matches!(
            derive_risk_class(action),
            RiskClass::Delete | RiskClass::Install | RiskClass::Update | RiskClass::ExternalEffect
        ) {
            reasons.push(format!(
                "{action} requires human approval before consequence"
            ));
            PolicyDecisionKind::NeedsApproval
        } else if matches!(
            derive_risk_class(action),
            RiskClass::Read | RiskClass::Diagnostic
        ) {
            PolicyDecisionKind::Ok
        } else {
            reasons.push("stub_policy_v0 cannot resolve action safely".to_string());
            PolicyDecisionKind::Ghost
        };
    PolicyDecision {
        decision_id: format!("pdec_{}", stable_id(actor, action, resource)),
        action_id: format!("actreq_{}", stable_id(actor, action, resource)),
        engine: "stub_policy_v0".to_string(),
        decision,
        reasons,
        matched_policy_ids: None,
        evaluated_at: now_string(),
    }
}

fn gate_decide(args: ActionArgs, as_json: bool) -> Result<()> {
    let policy = stub_policy_v0(&args.actor, &args.action, &args.resource);
    let actor = principal_from_actor(&args.actor);
    let resource = ResourceRef::new(ResourceKind::Lab, args.resource.clone());
    let output = if policy.decision == PolicyDecisionKind::NeedsApproval {
        let gate = GateRequest {
            gate_id: format!(
                "gate_{}",
                stable_id(&args.actor, &args.action, &args.resource)
            ),
            action_id: policy.action_id.clone(),
            actor,
            resource,
            policy_decision_id: policy.decision_id.clone(),
            status: GateStatus::Pending,
            requested_at: now_string(),
            expires_at: None,
            required_approver: Some("dan".to_string()),
            reason: policy
                .reasons
                .first()
                .cloned()
                .unwrap_or_else(|| "approval required".to_string()),
            execution_window: None,
        };
        validate_gate_request(&gate).map_err(anyhow::Error::msg)?;
        json!({ "policy_decision": policy, "gate_request": gate, "secret_values_printed": false })
    } else {
        json!({ "policy_decision": policy, "gate_request": null, "secret_values_printed": false })
    };
    if as_json {
        print_json(&output)
    } else {
        println!("{}", serde_json::to_string_pretty(&output)?);
        Ok(())
    }
}

fn principal_from_actor(actor: &str) -> PrincipalRef {
    let kind = if is_llm_actor(actor) {
        PrincipalKind::Llm
    } else {
        PrincipalKind::Human
    };
    PrincipalRef::new(actor, kind)
}

fn is_llm_actor(actor: &str) -> bool {
    matches!(
        actor.to_ascii_lowercase().as_str(),
        "llm" | "chatgpt" | "gpt" | "claude"
    )
}

fn stable_id(parts: &str, action: &str, resource: &str) -> String {
    let raw = format!("{parts}_{action}_{resource}");
    raw.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

pub fn now_string() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix_{secs}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{GhostKind, ReleaseArtifactStatus};

    #[test]
    fn secret_key_preferred_over_service_role_key() {
        let result = config_doctor_from_env(vec![
            "SUPABASE_SECRET_KEY".into(),
            "SUPABASE_SERVICE_ROLE_KEY".into(),
            "SUPABASE_PUBLISHABLE_KEY".into(),
            "SUPABASE_URL".into(),
        ]);
        assert!(result
            .canonical_keys_used
            .contains(&"SUPABASE_SECRET_KEY".to_string()));
        assert!(!result
            .legacy_keys_used
            .contains(&"SUPABASE_SERVICE_ROLE_KEY".to_string()));
        assert!(validate_config_doctor_result(&result).is_ok());
    }

    #[test]
    fn next_public_secret_key_fails() {
        let result = config_doctor_from_env(vec!["NEXT_PUBLIC_SUPABASE_SECRET_KEY".into()]);
        assert!(validate_config_doctor_result(&result).is_err());
    }

    #[test]
    fn derive_update_host_runtime_is_update() {
        assert_eq!(derive_risk_class("update_host_runtime"), RiskClass::Update);
    }

    #[test]
    fn policy_update_needs_approval() {
        assert_eq!(
            stub_policy_v0("dan", "update_host_runtime", "lab256").decision,
            PolicyDecisionKind::NeedsApproval
        );
    }

    #[test]
    fn policy_denies_llm_execute() {
        assert_eq!(
            stub_policy_v0("chatgpt", "execute", "lab256").decision,
            PolicyDecisionKind::Denied
        );
    }

    #[test]
    fn gate_for_update_is_pending() {
        let policy = stub_policy_v0("dan", "update_host_runtime", "lab256");
        let gate = GateRequest {
            gate_id: "gate_test".into(),
            action_id: policy.action_id,
            actor: PrincipalRef::new("dan", PrincipalKind::Human),
            resource: ResourceRef::new(ResourceKind::Lab, "lab256"),
            policy_decision_id: policy.decision_id,
            status: GateStatus::Pending,
            requested_at: now_string(),
            expires_at: None,
            required_approver: Some("dan".into()),
            reason: "update requires approval".into(),
            execution_window: None,
        };
        assert!(validate_gate_request(&gate).is_ok());
    }

    #[test]
    fn planned_release_artifact_is_not_installable() {
        let artifact = ReleaseArtifactRef {
            artifact_id: "a".into(),
            package_name: "pkg".into(),
            version: "0.1.0".into(),
            storage_bucket: "b".into(),
            storage_path: "p".into(),
            sha256: "".into(),
            size_bytes: None,
            status: ReleaseArtifactStatus::Planned,
        };
        assert!(validate_release_artifact_ref(&artifact).is_err());
    }

    #[test]
    fn lab_event_requires_secret_redacted() {
        let event = LabEvent {
            event_id: "e".into(),
            host_id: "lab256".into(),
            event_type: "heartbeat".into(),
            runtime_version: Some("0.1.0".into()),
            evidence: None,
            payload: None,
            secret_redacted: false,
            observed_at: now_string(),
        };
        assert!(validate_lab_event(&event).is_err());
    }

    #[test]
    fn authority_response_cannot_ok_with_ghosts() {
        let response = AuthorityResponse {
            ok: true,
            status: AuthorityStatus::Ok,
            message: "bad".into(),
            data: None,
            ghosts: vec![Ghost {
                ghost_id: "g".into(),
                kind: GhostKind::Unknown,
                subject: "x".into(),
                reason: "unknown".into(),
                detected_at: now_string(),
            }],
            errors: vec![],
            success_conditions: vec![],
            satisfied_conditions: vec![],
            secret_values_printed: false,
        };
        assert!(validate_authority_response(&response).is_err());
    }

    #[test]
    fn config_doctor_rejects_secret_values_printed() {
        let mut result = config_doctor_from_env(vec![]);
        result.secret_values_printed = true;
        assert!(validate_config_doctor_result(&result).is_err());
    }
}
