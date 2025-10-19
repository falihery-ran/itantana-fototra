use std::future::Future;

use anyhow::anyhow;

use crate::dtos::user_permission::user_permission_add_request::UserPermissionAddRequest;
use crate::dtos::user_permission::user_permission_delete_request::UserPermissionDeleteRequest;
use crate::dtos::user_permission::user_permission_find_request_filter::UserPermissionFindRequestFilter;
use crate::dtos::{find_request::FindRequest, find_response::FindResponse};
use crate::model::user_permission::UserPermission;
use crate::model::user_permission::error::UserPermissionError;
use crate::repository::user_permission_repository::UserPermissionRepository;
use crate::runtime::Runtime;
use crate::service::error::ServiceError;
use crate::traits::authentication_trait::AuthenticationTrait;

#[derive(Debug, Clone)]
pub struct UserPermissionService;

impl UserPermissionService {
    pub fn create(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserPermissionAddRequest,
    ) -> impl Future<Output = Result<UserPermission, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_permission:create")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get::<UserPermissionRepository>()
                .await
                .ok_or(ServiceError::new(UserPermissionError::Unknown(anyhow!(
                    "Cannot get user_permission repository"
                ))))?
                .clone()
                .save(&req.into())
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn find(
        authenticatable: &dyn AuthenticationTrait,
        req: &FindRequest<UserPermissionFindRequestFilter>,
    ) -> impl Future<Output = Result<FindResponse<UserPermission>, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_permission:find")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get::<UserPermissionRepository>()
                .await
                .ok_or(ServiceError::new(UserPermissionError::Unknown(anyhow!(
                    "Cannot get user_permisison repository"
                ))))?
                .clone()
                .find_all(req)
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn delete(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserPermissionDeleteRequest,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_permission:delete")
                .await
                .map_err(ServiceError::new)?;
            Runtime::get::<UserPermissionRepository>()
                .await
                .ok_or(ServiceError::new(UserPermissionError::Unknown(anyhow!(
                    "Cannot get user_permission repository"
                ))))?
                .clone()
                .delete(&(*req.get_user_id(), req.get_permission().clone()))
                .await
                .map_err(ServiceError::new)
        })
    }
}
