use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct UserPermissionDeleteRequest(uuid::Uuid, String);

impl UserPermissionDeleteRequest {
    pub fn new(user_id: &uuid::Uuid, permission: &str) -> Self {
        Self(*user_id, permission.to_string())
    }

    pub fn get_user_id(&self) -> &uuid::Uuid {
        &self.0
    }

    pub fn get_permission(&self) -> &str {
        &self.1
    }
}
