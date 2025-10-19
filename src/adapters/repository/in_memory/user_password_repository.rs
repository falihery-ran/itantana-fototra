use std::{
    collections::HashMap,
    pin::Pin,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use crate::{
    adapters::repository::in_memory::user_repository::InMemoryUserRepository,
    dtos::{find_request::FindRequest, find_response::FindResponse},
    model::{
        password::Password,
        user::{UserID, error::UserError},
        user_password::{UserPassword, error::UserPasswordError},
    },
    traits::{
        initialize_trait::InitializeTrait, repository_trait::RepositoryTrait,
        user_password::user_password_repository_trait::UserPasswordRepositoryTrait,
    },
};

static DB: LazyLock<Arc<RwLock<HashMap<UserID, String>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone)]
pub struct InMemoryUserPasswordRepository {
    data: Arc<RwLock<HashMap<UserID, String>>>,
    user_repository: InMemoryUserRepository,
}

impl Default for InMemoryUserPasswordRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryUserPasswordRepository {
    pub fn new() -> Self {
        Self {
            data: DB.clone(),
            user_repository: InMemoryUserRepository::new(),
        }
    }
}

impl RepositoryTrait for InMemoryUserPasswordRepository {
    type Id = UserID;
    type Entity = UserPassword;
    type Error = UserPasswordError;
    type FindOptions = FindRequest<()>;
    type FindResult = FindResponse<()>;

    fn save<'a>(
        &'a self,
        entity: &'a Self::Entity,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>>
    {
        Box::pin(async {
            self.user_repository
                .find_by_id(entity.get_user_id())
                .await
                .map_err(|e| match e {
                    UserError::UserNotExists { id } => UserPasswordError::UserNotExists { id },
                    ref e => UserPasswordError::Unknown(anyhow::anyhow!(e.to_string())),
                })?;
            let user_password = UserPassword::new(entity.get_user_id(), entity.get_password());
            let mut data = self.data.write().await;
            let _ = user_password
                .get_password()
                .hash()
                .map_err(UserPasswordError::PasswordError);
            data.insert(
                *entity.get_user_id(),
                user_password
                    .get_password()
                    .hash()
                    .map_err(UserPasswordError::PasswordError)?,
            );
            Ok(user_password)
        })
    }

    fn update<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
        _entity: &'a Self::Entity,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<UserPassword, UserPasswordError>> + Send + 'a>,
    > {
        Box::pin(async move { unimplemented!() })
    }

    fn delete<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), UserPasswordError>> + Send + 'a>> {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if data.contains_key(entity_id) {
                data.remove(entity_id);
            }
            Ok(())
        })
    }

    fn find_all<'a>(
        &'a self,
        _options: &'a Self::FindOptions,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<FindResponse<()>, UserPasswordError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async { unimplemented!() })
    }

    fn find_by_id<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<UserPassword, UserPasswordError>> + Send + 'a>,
    > {
        Box::pin(async { unimplemented!() })
    }
}

impl UserPasswordRepositoryTrait for InMemoryUserPasswordRepository {
    fn verify_password<'a>(
        &'a self,
        user_id: &'a Self::Id,
        password: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        Box::pin(async {
            let db = self.data.read().await;
            let hashed_password = db
                .get(user_id)
                .ok_or(UserPasswordError::UserNotExists { id: *user_id })?;
            Password::verify(password, hashed_password).map_err(UserPasswordError::PasswordError)
        })
    }
}

impl InitializeTrait for InMemoryUserPasswordRepository {
    fn initialize<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }
}
