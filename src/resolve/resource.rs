use crate::{
    adapters::{AuthorityError, SupabaseReadAdapter},
    contracts::*,
    resolve::principal::ghost_for_error,
};

pub fn resolve_resource<A: SupabaseReadAdapter + ?Sized>(
    adapter: &A,
    resource_id: &str,
) -> Result<ResourceRef, Ghost> {
    match adapter.get_lab_host(resource_id) {
        Ok(host) => {
            let mut resource = ResourceRef::new(ResourceKind::Lab, host.host_id);
            resource.schema = Some("lab_observability".to_string());
            resource.table = Some("hosts".to_string());
            Ok(resource)
        }
        Err(AuthorityError::MissingRegistryEntity) => match adapter.get_entity(resource_id) {
            Ok(entity) => {
                let mut resource = ResourceRef::new(ResourceKind::Entity, entity.entity_id);
                resource.schema = Some("registry".to_string());
                resource.table = Some("entities".to_string());
                Ok(resource)
            }
            Err(err) => Err(ghost_for_error(resource_id, err)),
        },
        Err(err) => Err(ghost_for_error(resource_id, err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake;
    impl SupabaseReadAdapter for Fake {
        fn get_entity(&self, _: &str) -> Result<RegistryEntityRef, AuthorityError> {
            Err(AuthorityError::MissingRegistryEntity)
        }
        fn get_lab_host(&self, host_id: &str) -> Result<LabHostRef, AuthorityError> {
            if host_id == "lab256" {
                Ok(LabHostRef {
                    host_id: "lab256".into(),
                    display_name: None,
                    runtime_entity_id: None,
                    runtime_version: None,
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
    fn lab256_found_as_lab_resource() {
        assert_eq!(
            resolve_resource(&Fake, "lab256").unwrap().kind,
            ResourceKind::Lab
        );
    }
    #[test]
    fn unknown_lab_produces_ghost() {
        assert_eq!(
            resolve_resource(&Fake, "lab404").unwrap_err().kind,
            GhostKind::MissingRegistryEntity
        );
    }
}
