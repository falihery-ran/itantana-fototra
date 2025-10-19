use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct UserPermissionFindRequestFilter {
    pub user_id: Option<uuid::Uuid>,
    pub permission: Option<String>,
}
