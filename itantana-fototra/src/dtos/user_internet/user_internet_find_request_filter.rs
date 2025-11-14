use serde::Deserialize;

use crate::model::user::UserID;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
pub struct UserInternetFindRequestFilter {
    pub user_id: Option<UserID>,
    pub email: Option<String>,
}
