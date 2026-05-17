use crate::{
    adapters::{AuthorityError, SupabaseReadAdapter},
    contracts::RegistryEntityRef,
};

pub struct RegistryRepository<'a, A: SupabaseReadAdapter + ?Sized> {
    adapter: &'a A,
}
impl<'a, A: SupabaseReadAdapter + ?Sized> RegistryRepository<'a, A> {
    pub fn new(adapter: &'a A) -> Self {
        Self { adapter }
    }
    pub fn get(&self, entity_id: &str) -> Result<RegistryEntityRef, AuthorityError> {
        self.adapter.get_entity(entity_id)
    }
}
