use serde::Deserialize;

use crate::model::user::UserID;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserPasswordAddRequest {
    user_id: UserID,
    password: String,
}

impl UserPasswordAddRequest {
    pub fn new(user_id: &UserID, password: &str) -> Self {
        Self {
            user_id: *user_id,
            password: password.to_string(),
        }
    }

    pub fn get_user_id(&self) -> &UserID {
        &self.user_id
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}
