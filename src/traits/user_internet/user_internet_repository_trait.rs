use crate::{
    dtos::{
        find_request::FindRequest, find_response::FindResponse,
        user_internet::user_internet_find_request_filter::UserInternetFindRequestFilter,
    },
    model::{
        email_address::EmailAddress,
        user::UserID,
        user_internet::{UserInternet, error::UserInternetError},
    },
    traits::{initialize_trait::InitializeTrait, repository_trait::RepositoryTrait},
};

pub trait UserInternetRepositoryTrait:
    InitializeTrait
    + RepositoryTrait<
        Id = (UserID, EmailAddress),
        Entity = UserInternet,
        Error = UserInternetError,
        FindOptions = FindRequest<UserInternetFindRequestFilter>,
        FindResult = FindResponse<UserInternet>,
    >
{
}
