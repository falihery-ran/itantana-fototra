use thiserror::Error;

use crate::model::user::UserID;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("The id {id1} in the request differ the id {id2}")]
    MismatchUserId { id1: UserID, id2: UserID },
    #[error("User with id {id} does not exists")]
    UserNotExists { id: UserID },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
