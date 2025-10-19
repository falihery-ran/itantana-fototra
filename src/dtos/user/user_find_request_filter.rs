use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserFindRequestFilter {
    pub id: Option<uuid::Uuid>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}
