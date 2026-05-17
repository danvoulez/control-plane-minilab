use std::str::FromStr;

use cedar_policy::{
    Authorizer, Context, Decision, Entities, EntityId, EntityTypeName, EntityUid, PolicySet,
    Request,
};

use crate::contracts::{PolicyDecisionKind, PrincipalRef, ResourceRef};

#[derive(Debug, Default, Clone)]
pub struct CedarPolicyAdapter;

impl CedarPolicyAdapter {
    pub fn evaluate_static_policy(
        &self,
        policy: &str,
        actor: &PrincipalRef,
        action: &str,
        resource: &ResourceRef,
    ) -> Result<PolicyDecisionKind, String> {
        let policies = PolicySet::from_str(policy).map_err(|err| err.to_string())?;
        let principal = uid("User", &actor.entity_id)?;
        let action = uid("Action", action)?;
        let resource = uid("Resource", &resource.id)?;
        let request = Request::new(principal, action, resource, Context::empty(), None)
            .map_err(|err| err.to_string())?;
        let entities = Entities::empty();
        let response = Authorizer::new().is_authorized(&request, &policies, &entities);
        Ok(match response.decision() {
            Decision::Allow => PolicyDecisionKind::Ok,
            Decision::Deny => PolicyDecisionKind::Denied,
        })
    }
}

fn uid(kind: &str, id: &str) -> Result<EntityUid, String> {
    let type_name = EntityTypeName::from_str(kind).map_err(|err| err.to_string())?;
    let entity_id = EntityId::from_str(id).map_err(|err| err.to_string())?;
    Ok(EntityUid::from_type_name_and_id(type_name, entity_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{PrincipalKind, ResourceKind};

    #[test]
    fn static_policy_returns_allow_or_deny() {
        let policy = r#"permit(principal == User::"dan", action == Action::"read", resource == Resource::"lab256");"#;
        let actor = PrincipalRef::new("dan", PrincipalKind::Human);
        let resource = ResourceRef::new(ResourceKind::Lab, "lab256");
        assert_eq!(
            CedarPolicyAdapter
                .evaluate_static_policy(policy, &actor, "read", &resource)
                .unwrap(),
            PolicyDecisionKind::Ok
        );
    }
}
