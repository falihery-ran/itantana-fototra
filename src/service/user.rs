use anyhow::anyhow;
use std::future::Future;

use crate::model::user::UserID;
use crate::runtime::Runtime;
use crate::service::error::ServiceError;
use crate::traits::authentication_trait::AuthenticationTrait;
use crate::{
    dtos::{
        find_request::FindRequest,
        find_response::FindResponse,
        user::{
            user_add_request::UserAddRequest, user_delete_request::UserDeleteRequest,
            user_find_request_filter::UserFindRequestFilter,
            user_update_request::UserUpdateRequest,
        },
    },
    model::user::{User, error::UserError},
    repository::user_repository::UserRepository,
};

#[derive(Debug, Clone)]
pub struct UserService;

impl UserService {
    pub fn create(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserAddRequest,
    ) -> impl Future<Output = Result<User, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user:create")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get_instance()
                .get::<UserRepository>()
                .await
                .ok_or(ServiceError::new(UserError::Unknown(anyhow!(
                    "Cannot get user repository"
                ))))?
                .clone()
                .save(&req.into())
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn update(
        authenticatable: &dyn AuthenticationTrait,
        user_id: &UserID,
        req: &UserUpdateRequest,
    ) -> impl Future<Output = Result<User, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user:update")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get_instance()
                .get::<UserRepository>()
                .await
                .ok_or(ServiceError::new(UserError::Unknown(anyhow!(
                    "Cannot get user repository"
                ))))?
                .clone()
                .update(user_id, &req.into())
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn find_one(
        authenticatable: &dyn AuthenticationTrait,
        user_id: &UserID,
    ) -> impl Future<Output = Result<User, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user:find_one")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get_instance()
                .get::<UserRepository>()
                .await
                .ok_or(ServiceError::new(UserError::Unknown(anyhow!(
                    "Cannot get user repository"
                ))))?
                .clone()
                .find_by_id(user_id)
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn find(
        authenticatable: &dyn AuthenticationTrait,
        req: &FindRequest<UserFindRequestFilter>,
    ) -> impl Future<Output = Result<FindResponse<User>, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user:find")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get_instance()
                .get::<UserRepository>()
                .await
                .ok_or(ServiceError::new(UserError::Unknown(anyhow!(
                    "Cannot get user repository"
                ))))?
                .clone()
                .find_all(req)
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn delete(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserDeleteRequest,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user:delete")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get_instance()
                .get::<UserRepository>()
                .await
                .ok_or(ServiceError::new(UserError::Unknown(anyhow!(
                    "Cannot get user repository"
                ))))?
                .clone()
                .delete(req.get_user_id())
                .await
                .map_err(ServiceError::new)
        })
    }
}
