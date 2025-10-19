use serde::{Deserialize, Serialize};

use crate::model::{permission::Permission, user::UserID};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPermissionFindRequestFilter {
    pub user_id: Option<UserID>,
    pub permission: Option<Permission>,
}
