use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::user::user_permission::UserPermission;

#[derive(Debug, PartialEq, Eq, Deserialize, ToSchema)]
pub struct UserPermissionAddRequest {
    user_id: Uuid,
    permission: String,
}

impl From<&UserPermissionAddRequest> for UserPermission {
    fn from(val: &UserPermissionAddRequest) -> Self {
        UserPermission::new(&val.user_id, &val.permission)
    }
}

impl UserPermissionAddRequest {
    pub fn new(user_id: &Uuid, permission: &str) -> Self {
        Self {
            user_id: *user_id,
            permission: permission.to_string(),
        }
    }

    pub fn get_user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn get_permission(&self) -> &str {
        &self.permission
    }
}
