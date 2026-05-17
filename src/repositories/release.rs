use crate::{
    adapters::{AuthorityError, SupabaseReadAdapter},
    contracts::ReleaseArtifactRef,
};

pub struct ReleaseRepository<'a, A: SupabaseReadAdapter + ?Sized> {
    adapter: &'a A,
}
impl<'a, A: SupabaseReadAdapter + ?Sized> ReleaseRepository<'a, A> {
    pub fn new(adapter: &'a A) -> Self {
        Self { adapter }
    }
    pub fn get_artifact(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<ReleaseArtifactRef, AuthorityError> {
        self.adapter.get_release_artifact(package_name, version)
    }
    pub fn latest_published(
        &self,
        package_name: &str,
    ) -> Result<ReleaseArtifactRef, AuthorityError> {
        self.adapter.get_latest_published_artifact(package_name)
    }
}
