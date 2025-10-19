pub mod error;
use serde::{Deserialize, Serialize};

use crate::model::password::password_level::PasswordLevel;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserPasswordPolicy(pub PasswordLevel);
