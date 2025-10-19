use thiserror::Error;

use crate::model::{password::PasswordError, user::UserID};

#[derive(Debug, Error)]
pub enum UserPasswordError {
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error("User with id {id} does not exists")]
    UserNotExists { id: UserID },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
