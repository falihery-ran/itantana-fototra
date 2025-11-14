use serde::Deserialize;
use uuid::Uuid;

use crate::model::user::{User, name::Name};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UserAddRequest {
    firstname: Name,
    lastname: Option<Name>,
}

impl From<&UserAddRequest> for User {
    fn from(val: &UserAddRequest) -> Self {
        User::new(&Uuid::nil(), &val.firstname, val.lastname.as_ref())
    }
}

impl UserAddRequest {
    pub fn new(firstname: &Name, lastname: Option<&Name>) -> Self {
        Self {
            firstname: firstname.clone(),
            lastname: lastname.cloned(),
        }
    }

    pub fn get_firstname(&self) -> &Name {
        &self.firstname
    }

    pub fn get_lastname(&self) -> Option<&Name> {
        self.lastname.as_ref()
    }
}
