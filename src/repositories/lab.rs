use crate::{
    adapters::{AuthorityError, SupabaseReadAdapter},
    contracts::LabHostRef,
};

pub struct LabRepository<'a, A: SupabaseReadAdapter + ?Sized> {
    adapter: &'a A,
}
impl<'a, A: SupabaseReadAdapter + ?Sized> LabRepository<'a, A> {
    pub fn new(adapter: &'a A) -> Self {
        Self { adapter }
    }
    pub fn get_host(&self, host_id: &str) -> Result<LabHostRef, AuthorityError> {
        self.adapter.get_lab_host(host_id)
    }
}
