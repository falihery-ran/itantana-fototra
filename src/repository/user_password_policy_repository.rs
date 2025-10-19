use std::{ops::Deref, sync::Arc};

use crate::traits::user_password::user_password_policy::user_password_policy_repository::UserPasswordPolicyRepositoryTrait;

pub struct UserPasswordPolicyRepository {
    inner: Arc<dyn UserPasswordPolicyRepositoryTrait>,
}

impl UserPasswordPolicyRepository {
    pub fn new(
        user_password_policy_repository: Arc<dyn UserPasswordPolicyRepositoryTrait>,
    ) -> Self {
        Self {
            inner: user_password_policy_repository.clone(),
        }
    }
}

impl Deref for UserPasswordPolicyRepository {
    type Target = dyn UserPasswordPolicyRepositoryTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
