pub mod error;
pub mod user_password_policy;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::model::password::Password;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct UserPassword {
    user_id: Uuid,
    password: Password,
    updated_at: DateTime<Utc>,
}

impl UserPassword {
    pub fn new(user_id: &Uuid, password: &Password) -> Self {
        Self {
            user_id: *user_id,
            password: password.clone(),
            updated_at: Utc::now(),
        }
    }

    pub fn get_user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn get_password(&self) -> &Password {
        &self.password
    }

    pub fn get_updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
