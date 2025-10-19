use std::{ops::Deref, sync::Arc};

use crate::traits::user_internet::user_internet_repository_trait::UserInternetRepositoryTrait;

pub struct UserInternetRepository {
    inner: Arc<dyn UserInternetRepositoryTrait>,
}

impl UserInternetRepository {
    pub fn new(user_internet_repository: Arc<dyn UserInternetRepositoryTrait>) -> Self {
        Self {
            inner: user_internet_repository.clone(),
        }
    }
}

impl Deref for UserInternetRepository {
    type Target = dyn UserInternetRepositoryTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
