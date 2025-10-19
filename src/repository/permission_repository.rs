use std::{ops::Deref, sync::Arc};

use crate::traits::permission::permission_repository_trait::PermissionRepositoryTrait;

pub struct PermissionRepository {
    inner: Arc<dyn PermissionRepositoryTrait>,
}

impl PermissionRepository {
    pub fn new(user_repository: Arc<dyn PermissionRepositoryTrait>) -> Self {
        Self {
            inner: user_repository.clone(),
        }
    }
}

impl Deref for PermissionRepository {
    type Target = dyn PermissionRepositoryTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
