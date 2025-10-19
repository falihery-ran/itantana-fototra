use crate::{
    dtos::{
        find_request::FindRequest, find_response::FindResponse,
        user::user_find_request_filter::UserFindRequestFilter,
    },
    model::user::{User, UserID, error::UserError},
    traits::{initialize_trait::InitializeTrait, repository_trait::RepositoryTrait},
};

pub trait UserRepositoryTrait:
    InitializeTrait
    + RepositoryTrait<
        Id = UserID,
        Entity = User,
        Error = UserError,
        FindOptions = FindRequest<UserFindRequestFilter>,
        FindResult = FindResponse<User>,
    > + Sync
    + Send
    + 'static
{
}
