use uuid::Uuid;

use crate::{
    dtos::{
        find_request::FindRequest, find_response::FindResponse,
        user::user_permission::user_permission_find_request_filter::UserPermissionFindRequestFilter,
    },
    model::user::user_permission::{UserPermission, error::UserPermissionError},
    ports::repository_trait::RepositoryTrait,
};

pub trait UserPermissionRepositoryTrait:
    RepositoryTrait<
        Id = (Uuid, String),
        Entity = UserPermission,
        Error = UserPermissionError,
        FindOptions = FindRequest<UserPermissionFindRequestFilter>,
        FindResult = FindResponse<UserPermission>,
    >
{
}
