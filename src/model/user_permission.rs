pub mod error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{permission::Permission, user::UserID};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserPermission {
    user_id: UserID,
    permission: Permission,
}

impl UserPermission {
    pub fn new(user_id: &uuid::Uuid, permisison: &str) -> Self {
        Self {
            user_id: *user_id,
            permission: permisison.to_string(),
        }
    }

    pub fn get_user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn get_permission(&self) -> &str {
        &self.permission
    }
}
