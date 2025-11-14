use serde::Deserialize;

use crate::model::{permission::Permission, user::UserID};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UserPermissionDeleteRequest(UserID, Permission);

impl UserPermissionDeleteRequest {
    pub fn new(user_id: &UserID, permission: &str) -> Self {
        Self(*user_id, permission.to_string())
    }

    pub fn get_user_id(&self) -> &UserID {
        &self.0
    }

    pub fn get_permission(&self) -> &Permission {
        &self.1
    }
}
