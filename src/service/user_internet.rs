use anyhow::anyhow;

use crate::{
    dtos::{
        find_request::FindRequest,
        find_response::FindResponse,
        user_internet::{
            user_internet_add_request::UserInternetAddRequest,
            user_internet_delete_request::UserInternetDeleteRequest,
            user_internet_find_request_filter::UserInternetFindRequestFilter,
        },
    },
    model::user_internet::{UserInternet, error::UserInternetError},
    repository::user_internet_repository::UserInternetRepository,
    runtime::Runtime,
    service::error::ServiceError,
    traits::authentication_trait::AuthenticationTrait,
};

#[derive(Debug, Clone)]
pub struct UserInternetService;

impl UserInternetService {
    pub fn create(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserInternetAddRequest,
    ) -> impl Future<Output = Result<UserInternet, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_internet:create")
                .await
                .map_err(ServiceError::new)?;

            Runtime::get_instance()
                .get::<UserInternetRepository>()
                .await
                .ok_or(ServiceError::new(UserInternetError::Unknown(anyhow!(
                    "Cannot get user_internet repository"
                ))))?
                .clone()
                .save(&req.into())
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn find(
        authenticatable: &dyn AuthenticationTrait,
        req: &FindRequest<UserInternetFindRequestFilter>,
    ) -> impl Future<Output = Result<FindResponse<UserInternet>, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_internet:find")
                .await
                .map_err(ServiceError::new)?;

            Runtime::get_instance()
                .get::<UserInternetRepository>()
                .await
                .ok_or(ServiceError::new(UserInternetError::Unknown(anyhow!(
                    "Cannot get user_internet repository"
                ))))?
                .clone()
                .find_all(req)
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn delete(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserInternetDeleteRequest,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_internet:delete")
                .await
                .map_err(ServiceError::new)?;

            Runtime::get_instance()
                .get::<UserInternetRepository>()
                .await
                .ok_or(ServiceError::new(UserInternetError::Unknown(anyhow!(
                    "Cannot get user_internet repository"
                ))))?
                .clone()
                .delete(&(*req.get_user_id(), req.get_email().clone()))
                .await
                .map_err(ServiceError::new)
        })
    }
}
