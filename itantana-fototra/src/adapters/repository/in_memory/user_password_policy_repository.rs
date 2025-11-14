use std::{
    pin::Pin,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use crate::{
    dtos::{find_request::FindRequest, find_response::FindResponse},
    model::{
        password::password_level::PasswordLevel,
        user_password::user_password_policy::{UserPasswordPolicy, error::UserPasswordPolicyError},
    },
    traits::{
        initialize_trait::InitializeTrait, repository_trait::RepositoryTrait,
        user_password::user_password_policy::user_password_policy_repository::UserPasswordPolicyRepositoryTrait,
    },
};

static DB: LazyLock<Arc<RwLock<PasswordLevel>>> =
    LazyLock::new(|| Arc::new(RwLock::new(PasswordLevel::default())));

#[derive(Debug, Clone)]
pub struct InMemoryUserPasswordPolicyRepository {
    data: Arc<RwLock<PasswordLevel>>,
}

impl Default for InMemoryUserPasswordPolicyRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryUserPasswordPolicyRepository {
    pub fn new() -> Self {
        Self { data: DB.clone() }
    }
}

impl RepositoryTrait for InMemoryUserPasswordPolicyRepository {
    type Id = PasswordLevel;
    type Entity = UserPasswordPolicy;
    type Error = UserPasswordPolicyError;
    type FindOptions = FindRequest<String>;
    type FindResult = FindResponse<UserPasswordPolicy>;

    fn delete<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        unimplemented!()
    }

    fn find_all<'a>(
        &'a self,
        _options: &'a Self::FindOptions,
    ) -> Pin<Box<dyn Future<Output = Result<Self::FindResult, Self::Error>> + Send + 'a>> {
        unimplemented!()
    }

    fn find_by_id<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>> {
        unimplemented!()
    }

    fn save<'a>(
        &'a self,
        _entity: &'a Self::Entity,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>> {
        unimplemented!()
    }

    fn update<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
        _entity: &'a Self::Entity,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>> {
        unimplemented!()
    }
}

impl UserPasswordPolicyRepositoryTrait for InMemoryUserPasswordPolicyRepository {
    fn change_policy<'a>(
        &'a self,
        password_level: &'a PasswordLevel,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        Box::pin(async {
            let mut db = self.data.write().await;
            *db = password_level.clone();
            Ok(())
        })
    }

    fn get_policy<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<UserPasswordPolicy, Self::Error>> + Send + 'a>> {
        Box::pin(async {
            let db = self.data.read().await;
            Ok(UserPasswordPolicy(db.clone()))
        })
    }
}

impl InitializeTrait for InMemoryUserPasswordPolicyRepository {
    fn initialize<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async {
            let mut db = self.data.write().await;
            *db = PasswordLevel::default();
            Ok(())
        })
    }
}
