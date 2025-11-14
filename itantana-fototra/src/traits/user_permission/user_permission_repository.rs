use crate::{
    dtos::{
        find_request::FindRequest, find_response::FindResponse,
        user_permission::user_permission_find_request_filter::UserPermissionFindRequestFilter,
    },
    model::{
        permission::Permission,
        user::UserID,
        user_permission::{UserPermission, error::UserPermissionError},
    },
    traits::{initialize_trait::InitializeTrait, repository_trait::RepositoryTrait},
};

pub trait UserPermissionRepositoryTrait:
    InitializeTrait
    + RepositoryTrait<
        Id = (UserID, Permission),
        Entity = UserPermission,
        Error = UserPermissionError,
        FindOptions = FindRequest<UserPermissionFindRequestFilter>,
        FindResult = FindResponse<UserPermission>,
    >
{
}
