use serde::Deserialize;

use crate::model::{email_address::EmailAddress, user::UserID, user_internet::UserInternet};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserInternetAddRequest {
    user_id: UserID,
    email: EmailAddress,
}

impl From<&UserInternetAddRequest> for UserInternet {
    fn from(val: &UserInternetAddRequest) -> Self {
        UserInternet::new(&val.user_id, &val.email)
    }
}

impl UserInternetAddRequest {
    pub fn new(user_id: &UserID, email: &EmailAddress) -> Self {
        Self {
            user_id: *user_id,
            email: email.clone(),
        }
    }

    pub fn get_user_id(&self) -> &UserID {
        &self.user_id
    }

    pub fn get_email(&self) -> &EmailAddress {
        &self.email
    }
}
