use crate::{
    adapters::{AuthorityError, SupabaseReadAdapter},
    contracts::*,
};

pub fn resolve_principal<A: SupabaseReadAdapter + ?Sized>(
    adapter: &A,
    actor_id: &str,
) -> Result<PrincipalRef, Ghost> {
    match adapter.get_entity(actor_id) {
        Ok(entity) => {
            let kind = match entity.kind {
                RegistryEntityKind::Llm => PrincipalKind::Llm,
                RegistryEntityKind::Service => PrincipalKind::Service,
                RegistryEntityKind::Computer => PrincipalKind::Lab,
                RegistryEntityKind::Runtime => PrincipalKind::Runtime,
                _ => PrincipalKind::Human,
            };
            let mut principal = PrincipalRef::new(entity.entity_id, kind);
            principal.display_name = entity.name;
            Ok(principal)
        }
        Err(err) => Err(ghost_for_error(actor_id, err)),
    }
}

pub fn ghost_for_error(subject: &str, err: AuthorityError) -> Ghost {
    let kind = match err {
        AuthorityError::MissingRegistryEntity => GhostKind::MissingRegistryEntity,
        AuthorityError::MissingConfigKey | AuthorityError::DatabaseAuthUnconfigured => {
            GhostKind::MissingConfig
        }
        AuthorityError::ForbiddenConfig => GhostKind::ForbiddenConfig,
        AuthorityError::ReleaseUnpublished => GhostKind::ReleaseUnpublished,
        AuthorityError::DatabaseAuthFailed => GhostKind::AuthUnverified,
        _ => GhostKind::Unknown,
    };
    Ghost {
        ghost_id: format!("ghost_{}_{}", err.code(), sanitize(subject)),
        kind,
        subject: subject.to_string(),
        reason: err.code().to_string(),
        detected_at: crate::cli::now_string(),
    }
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::AuthorityError;

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
        fn get_lab_host(&self, _: &str) -> Result<LabHostRef, AuthorityError> {
            Err(AuthorityError::MissingRegistryEntity)
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
    fn actor_found_produces_principal_ref() {
        assert_eq!(resolve_principal(&Fake, "dan").unwrap().entity_id, "dan");
    }
    #[test]
    fn unknown_actor_produces_ghost() {
        assert_eq!(
            resolve_principal(&Fake, "nobody").unwrap_err().kind,
            GhostKind::MissingRegistryEntity
        );
    }
}
