use std::{ops::Deref, sync::Arc};

use crate::traits::authorization_trait::AuthorizationTrait;

#[derive(Clone)]
pub struct Authorizable {
    inner: Arc<dyn AuthorizationTrait>,
}

impl Authorizable {
    pub fn new(authorizable: Arc<dyn AuthorizationTrait>) -> Self {
        Self {
            inner: authorizable.clone(),
        }
    }
}

impl Deref for Authorizable {
    type Target = dyn AuthorizationTrait;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
