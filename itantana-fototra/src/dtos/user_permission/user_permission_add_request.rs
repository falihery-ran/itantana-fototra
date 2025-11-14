use serde::Deserialize;

use crate::model::{permission::Permission, user::UserID, user_permission::UserPermission};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserPermissionAddRequest {
    user_id: UserID,
    permission: Permission,
}

impl From<&UserPermissionAddRequest> for UserPermission {
    fn from(val: &UserPermissionAddRequest) -> Self {
        UserPermission::new(&val.user_id, &val.permission)
    }
}

impl UserPermissionAddRequest {
    pub fn new(user_id: &UserID, permission: &str) -> Self {
        Self {
            user_id: *user_id,
            permission: permission.to_string(),
        }
    }

    pub fn get_user_id(&self) -> &UserID {
        &self.user_id
    }

    pub fn get_permission(&self) -> &str {
        &self.permission
    }
}
