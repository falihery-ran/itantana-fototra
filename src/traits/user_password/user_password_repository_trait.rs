use std::pin::Pin;

use crate::{
    dtos::{find_request::FindRequest, find_response::FindResponse},
    model::{
        user::UserID,
        user_password::{UserPassword, error::UserPasswordError},
    },
    traits::{initialize_trait::InitializeTrait, repository_trait::RepositoryTrait},
};

pub trait UserPasswordRepositoryTrait:
    InitializeTrait
    + RepositoryTrait<
        Id = UserID,
        Entity = UserPassword,
        Error = UserPasswordError,
        FindOptions = FindRequest<()>,
        FindResult = FindResponse<()>,
    >
{
    fn verify_password<'a>(
        &'a self,
        user_id: &'a Self::Id,
        password: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;
}
