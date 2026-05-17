use serde_json::json;

use crate::{
    adapters::SupabaseReadAdapter,
    cli::stub_policy_v0,
    contracts::*,
    resolve::{principal::resolve_principal, resource::resolve_resource},
    validate::{derive_risk_class, validate_authority_response},
};

pub fn resolve_action<A: SupabaseReadAdapter + ?Sized>(
    adapter: &A,
    actor_id: &str,
    verb: &str,
    resource_id: &str,
) -> AuthorityResponse {
    let mut ghosts = Vec::new();
    let actor = match resolve_principal(adapter, actor_id) {
        Ok(actor) => Some(actor),
        Err(ghost) => {
            ghosts.push(ghost);
            None
        }
    };
    let resource = match resolve_resource(adapter, resource_id) {
        Ok(resource) => Some(resource),
        Err(ghost) => {
            ghosts.push(ghost);
            None
        }
    };
    if !ghosts.is_empty() {
        return response(
            false,
            AuthorityStatus::Ghost,
            "unknowns resolved as ghosts",
            None,
            ghosts,
            vec![],
        );
    }
    let actor = actor.expect("actor checked");
    let resource = resource.expect("resource checked");
    let risk_class = derive_risk_class(verb);
    let policy = stub_policy_v0(&actor.entity_id, verb, &resource.id);
    let status = match policy.decision {
        PolicyDecisionKind::Ok => AuthorityStatus::Ok,
        PolicyDecisionKind::NeedsApproval => AuthorityStatus::Warn,
        PolicyDecisionKind::Denied | PolicyDecisionKind::Error => AuthorityStatus::Error,
        PolicyDecisionKind::Ghost => AuthorityStatus::Ghost,
    };
    let ok = status == AuthorityStatus::Ok;
    let output = response(
        ok,
        status,
        "action resolved with stub_policy_v0",
        Some(
            json!({ "actor": actor, "resource": resource, "risk_class": risk_class, "policy_decision": policy }),
        ),
        vec![],
        vec![],
    );
    let _ = validate_authority_response(&output);
    output
}

pub fn response(
    ok: bool,
    status: AuthorityStatus,
    message: &str,
    data: Option<serde_json::Value>,
    ghosts: Vec<Ghost>,
    errors: Vec<String>,
) -> AuthorityResponse {
    AuthorityResponse {
        ok,
        status,
        message: message.to_string(),
        data,
        ghosts,
        errors,
        success_conditions: vec!["known_actor".into(), "known_resource".into()],
        satisfied_conditions: if ok {
            vec!["known_actor".into(), "known_resource".into()]
        } else {
            vec![]
        },
        secret_values_printed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::AuthorityError, validate::validate_no_secret_values_in_json};

    struct Fake;
    impl SupabaseReadAdapter for Fake {
        fn get_entity(&self, entity_id: &str) -> Result<RegistryEntityRef, AuthorityError> {
            if entity_id == "dan" {
                Ok(RegistryEntityRef {
                    entity_id: "dan".into(),
                    name: Some("Dan".into()),
                    kind: RegistryEntityKind::Person,
                    status: RegistryEntityStatus::Active,
                    evidence_status: EvidenceMode::Observed,
                })
            } else {
                Err(AuthorityError::MissingRegistryEntity)
            }
        }

        fn get_lab_host(&self, host_id: &str) -> Result<LabHostRef, AuthorityError> {
            if host_id == "lab256" {
                Ok(LabHostRef {
                    host_id: "lab256".into(),
                    display_name: Some("LAB 256".into()),
                    runtime_entity_id: Some("minilab-host-runtime".into()),
                    runtime_version: Some("0.1.0".into()),
                    status: "online".into(),
                })
            } else {
                Err(AuthorityError::MissingRegistryEntity)
            }
        }

        fn get_release_artifact(
            &self,
            _: &str,
            _: &str,
        ) -> Result<ReleaseArtifactRef, AuthorityError> {
            Err(AuthorityError::MissingReleaseArtifact)
        }

        fn get_latest_published_artifact(
            &self,
            _: &str,
        ) -> Result<ReleaseArtifactRef, AuthorityError> {
            Err(AuthorityError::MissingReleaseArtifact)
        }

        fn get_config_key(&self, _: &str) -> Result<ConfigKeyRef, AuthorityError> {
            Err(AuthorityError::MissingConfigKey)
        }

        fn get_runtime_config_requirements(&self, _: &str) -> Result<Vec<String>, AuthorityError> {
            Ok(vec![])
        }
    }

    #[test]
    fn resolved_action_output_passes_secret_scan() {
        let output = resolve_action(&Fake, "dan", "update_host_runtime", "lab256");
        let value = serde_json::to_value(output).unwrap();
        validate_no_secret_values_in_json(&value).unwrap();
    }
}
