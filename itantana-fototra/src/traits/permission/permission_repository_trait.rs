use crate::{
    dtos::{find_request::FindRequest, find_response::FindResponse},
    model::permission::{Permission, error::PermissionError},
    traits::{initialize_trait::InitializeTrait, repository_trait::RepositoryTrait},
};

pub trait PermissionRepositoryTrait:
    InitializeTrait
    + RepositoryTrait<
        Id = String,
        Entity = Permission,
        Error = PermissionError,
        FindOptions = FindRequest<String>,
        FindResult = FindResponse<Permission>,
    > + Sync
    + Send
    + 'static
{
}
