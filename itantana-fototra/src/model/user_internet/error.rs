use thiserror::Error;

use crate::model::{email_address::EmailAddress, user::UserID};

#[derive(Debug, Error)]
pub enum UserInternetError {
    #[error("User with id {id} does not exists")]
    UserNotExists { id: UserID },
    #[error("Email {email} not associated to user id {user_id}")]
    EmailNotAssociatedToUser {
        email: EmailAddress,
        user_id: UserID,
    },
    #[error("Email address {email} already used")]
    EmailAlreadyUsed { email: EmailAddress },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
