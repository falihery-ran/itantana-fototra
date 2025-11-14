use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UserDeleteRequest(uuid::Uuid);

impl UserDeleteRequest {
    pub fn new(user_id: &uuid::Uuid) -> Self {
        Self(*user_id)
    }

    pub fn get_user_id(&self) -> &uuid::Uuid {
        &self.0
    }
}
