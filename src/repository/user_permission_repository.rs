use std::{ops::Deref, sync::Arc};

use crate::traits::user_permission::user_permission_repository::UserPermissionRepositoryTrait;

pub struct UserPermissionRepository {
    inner: Arc<dyn UserPermissionRepositoryTrait>,
}

impl UserPermissionRepository {
    pub fn new(user_permission_repository: Arc<dyn UserPermissionRepositoryTrait>) -> Self {
        Self {
            inner: user_permission_repository.clone(),
        }
    }
}

impl Deref for UserPermissionRepository {
    type Target = dyn UserPermissionRepositoryTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
