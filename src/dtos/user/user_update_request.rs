use serde::Deserialize;

use crate::model::user::{User, name::Name};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UserUpdateRequest {
    id: uuid::Uuid,
    firstname: Name,
    lastname: Option<Name>,
}

impl UserUpdateRequest {
    pub fn new(id: &uuid::Uuid, firstname: &Name, lastname: Option<&Name>) -> Self {
        Self {
            id: *id,
            firstname: firstname.clone(),
            lastname: lastname.cloned(),
        }
    }
}

impl From<&UserUpdateRequest> for User {
    fn from(val: &UserUpdateRequest) -> Self {
        User::new(&val.id, &val.firstname, val.lastname.as_ref())
    }
}
