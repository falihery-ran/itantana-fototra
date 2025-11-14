use std::{ops::Deref, sync::Arc};

use crate::traits::user::user_repository_trait::UserRepositoryTrait;

pub struct UserRepository {
    inner: Arc<dyn UserRepositoryTrait>,
}

impl UserRepository {
    pub fn new(user_repository: Arc<dyn UserRepositoryTrait>) -> Self {
        Self {
            inner: user_repository.clone(),
        }
    }
}

impl Deref for UserRepository {
    type Target = dyn UserRepositoryTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
