use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::user::UserID;

pub type ServiceAccountID = Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ServiceAccount {
    id: ServiceAccountID,
    name: String,
    description: Option<String>,
    data: String,
    created_by: UserID,
}
