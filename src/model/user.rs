pub mod error;
pub mod name;

use std::{str::FromStr, sync::LazyLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::user::name::Name;

pub type UserID = Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct User {
    id: UserID,
    lastname: Option<Name>,
    firstname: Name,
}

impl User {
    pub fn new(id: &UserID, firstname: &Name, lastname: Option<&Name>) -> Self {
        Self {
            id: *id,
            firstname: firstname.clone(),
            lastname: lastname.cloned(),
        }
    }

    pub fn get_id(&self) -> &UserID {
        &self.id
    }

    pub fn get_firstname(&self) -> &Name {
        &self.firstname
    }

    pub fn get_lastname(&self) -> Option<&Name> {
        self.lastname.as_ref()
    }
}

pub static DEFAULT_ADMIN_USER: LazyLock<User> = LazyLock::new(|| {
    User::new(
        &Uuid::from_str("f88f5dd0-d9c9-43fe-8791-7110d9d1cead").unwrap(),
        &Name::new("Admin").unwrap(),
        Some(&Name::new("Admin").unwrap()),
    )
});

// pub static ref DEFAULT_ADMIN_USER: Arc<User> =  Arc::new(User::new(
//             &Uuid::from_str("f88f5dd0-d9c9-43fe-8791-7110d9d1cead").unwrap(),
//             &Name::new("Falihery Emile").unwrap(),
//             Some(&Name::new("RANDRIANASOLO").unwrap()),
//         ));

// impl AuthorizationTrait for User {
//     fn authorize<'a>(
//             &'a self,
//             permission: &str,
//         ) -> std::pin::Pin<Box< dyn Future<Output = Result<(), crate::security::error::SecurityError>> + Send + 'a>> {
//         Runtime::get::<UserPermissionRepository>().await?;
//     }
// }
