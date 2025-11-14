use serde::Deserialize;

use crate::model::{email_address::EmailAddress, user::UserID};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UserInternetDeleteRequest(UserID, EmailAddress);

impl UserInternetDeleteRequest {
    pub fn new(user_id: &UserID, email: &EmailAddress) -> Self {
        Self(*user_id, email.clone())
    }

    pub fn get_user_id(&self) -> &uuid::Uuid {
        &self.0
    }

    pub fn get_email(&self) -> &EmailAddress {
        &self.1
    }
}
