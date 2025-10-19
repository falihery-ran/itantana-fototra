pub mod error;
use serde::{Deserialize, Serialize};

use crate::model::{email_address::EmailAddress, user::UserID};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserInternet {
    user_id: UserID,
    email: EmailAddress,
}

impl UserInternet {
    pub fn new(user_id: &uuid::Uuid, email: &EmailAddress) -> Self {
        Self {
            user_id: *user_id,
            email: email.clone(),
        }
    }

    pub fn get_user_id(&self) -> &uuid::Uuid {
        &self.user_id
    }

    pub fn get_email(&self) -> &EmailAddress {
        &self.email
    }
}
