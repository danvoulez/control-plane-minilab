use crate::{
    adapters::{AuthorityError, SupabaseReadAdapter},
    contracts::ConfigKeyRef,
};

pub struct ConfigRegistryRepository<'a, A: SupabaseReadAdapter + ?Sized> {
    adapter: &'a A,
}
impl<'a, A: SupabaseReadAdapter + ?Sized> ConfigRegistryRepository<'a, A> {
    pub fn new(adapter: &'a A) -> Self {
        Self { adapter }
    }
    pub fn get_key(&self, key_name: &str) -> Result<ConfigKeyRef, AuthorityError> {
        self.adapter.get_config_key(key_name)
    }
    pub fn runtime_requirements(
        &self,
        runtime_entity_id: &str,
    ) -> Result<Vec<String>, AuthorityError> {
        self.adapter
            .get_runtime_config_requirements(runtime_entity_id)
    }
}
