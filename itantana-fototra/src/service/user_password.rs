use anyhow::anyhow;

use crate::{
    dtos::user_password::user_password_add_request::UserPasswordAddRequest,
    model::{
        password::Password,
        user::UserID,
        user_password::{
            UserPassword, error::UserPasswordError,
            user_password_policy::error::UserPasswordPolicyError,
        },
    },
    registry::Registry,
    repository::{
        user_password_policy_repository::UserPasswordPolicyRepository,
        user_password_repository::UserPasswordRepository,
    },
    service::error::ServiceError,
    traits::authentication_trait::AuthenticationTrait,
};

#[derive(Debug, Clone)]
pub struct UserPasswordService;

impl UserPasswordService {
    pub fn create(
        authenticatable: &dyn AuthenticationTrait,
        req: &UserPasswordAddRequest,
    ) -> impl Future<Output = Result<UserPassword, ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_password:create")
                .await
                .map_err(ServiceError::new)?;
            let password = Password::new(
                req.get_password(),
                &Registry::get_instance()
                    .get::<UserPasswordPolicyRepository>("user_password_policy_repository")
                    .map_err(ServiceError::new)?
                    .ok_or(ServiceError::new(UserPasswordPolicyError::Unknown(
                        anyhow!("Cannot get user_password_policy repository"),
                    )))?
                    .clone()
                    .get_policy()
                    .await
                    .map_err(|e| ServiceError::new(UserPasswordError::Unknown(anyhow!(e))))?
                    .0,
            )
            .map_err(|e| ServiceError::new(UserPasswordError::PasswordError(e)))?;
            let user_password = UserPassword::new(req.get_user_id(), &password);
            Registry::get_instance()
                .get::<UserPasswordRepository>("user_password_repository")
                .map_err(ServiceError::new)?
                .ok_or(ServiceError::new(UserPasswordError::Unknown(anyhow!(
                    "Cannot get user_password repository"
                ))))?
                .clone()
                .save(&user_password)
                .await
                .map_err(ServiceError::new)
        })
    }

    pub fn match_user_password(
        authenticatable: &dyn AuthenticationTrait,
        user_id: &UserID,
        password: &str,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send {
        Box::pin(async {
            let authorizable = authenticatable
                .authenticate()
                .await
                .map_err(ServiceError::new)?;
            authorizable
                .authorize("user_password:match")
                .await
                .map_err(ServiceError::new)?;
            Registry::get_instance()
                .get::<UserPasswordRepository>("user_password_repository")
                .map_err(ServiceError::new)?
                .ok_or(ServiceError::new(UserPasswordError::Unknown(anyhow!(
                    "Cannot get user_password repository"
                ))))?
                .clone()
                .verify_password(user_id, password)
                .await
                .map_err(ServiceError::new)
        })
    }
}
