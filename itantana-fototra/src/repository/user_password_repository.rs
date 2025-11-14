use std::{ops::Deref, sync::Arc};

use crate::traits::user_password::user_password_repository_trait::UserPasswordRepositoryTrait;

pub struct UserPasswordRepository {
    inner: Arc<dyn UserPasswordRepositoryTrait>,
}

impl UserPasswordRepository {
    pub fn new(user_password_repository: Arc<dyn UserPasswordRepositoryTrait>) -> Self {
        Self {
            inner: user_password_repository.clone(),
        }
    }
}

impl Deref for UserPasswordRepository {
    type Target = dyn UserPasswordRepositoryTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
