use thiserror::Error;

use crate::model::{permission::Permission, user::UserID};

#[derive(Debug, Error)]
pub enum UserPermissionError {
    #[error("User with id {id} does not exists")]
    UserNotExists { id: UserID },
    #[error("Permission {permission} does not exist")]
    PermissionNotExists { permission: Permission },
    #[error("Permission {permission} is already assigned")]
    PermissionAlreadyAssigned { permission: Permission },
    #[error("Permission {permission} is already not assigned")]
    PermissionAlreadyNotAssigned { permission: Permission },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
