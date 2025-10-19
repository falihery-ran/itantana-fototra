pub mod permission_repository;
pub mod user_internet_repository;
pub mod user_password_policy_repository;
pub mod user_password_repository;
pub mod user_permission_repository;
pub mod user_repository;

use std::sync::Arc;

use crate::adapters::repository::in_memory::permission_repository::InMemoryPermissionRepository;
use crate::adapters::repository::in_memory::user_internet_repository::InMemoryUserInternetRepository;
use crate::adapters::repository::in_memory::user_password_policy_repository::InMemoryUserPasswordPolicyRepository;
use crate::adapters::repository::in_memory::user_password_repository::InMemoryUserPasswordRepository;
use crate::adapters::repository::in_memory::user_permission_repository::InMemoryUserPermissionRepository;
use crate::adapters::repository::in_memory::user_repository::InMemoryUserRepository;
use crate::repository::permission_repository::PermissionRepository;
use crate::repository::user_internet_repository::UserInternetRepository;
use crate::repository::user_password_policy_repository::UserPasswordPolicyRepository;
use crate::repository::user_password_repository::UserPasswordRepository;
use crate::repository::user_permission_repository::UserPermissionRepository;
use crate::repository::user_repository::UserRepository;
use crate::traits::adapter_loader_trait::AdapterLoaderTrait;

use crate::runtime::Runtime;
use crate::traits::initialize_trait::InitializeTrait;

#[derive(Debug)]
pub struct InMemoryRepository;

impl InitializeTrait for InMemoryRepository {
    fn initialize<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async {
            Runtime::get::<UserRepository>()
                .await
                .unwrap()
                .initialize()
                .await
                .unwrap();
            Ok(())
        })
    }
}

impl AdapterLoaderTrait for InMemoryRepository {
    fn name(&self) -> &str {
        "InMemoryRepository"
    }

    fn load(&self) -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        Box::pin(async {
            Runtime::register(PermissionRepository::new(Arc::new(
                InMemoryPermissionRepository::new(),
            )))
            .await;
            Runtime::register(UserRepository::new(Arc::new(InMemoryUserRepository::new()))).await;
            Runtime::register(UserInternetRepository::new(Arc::new(
                InMemoryUserInternetRepository::new(),
            )))
            .await;
            Runtime::register(UserPermissionRepository::new(Arc::new(
                InMemoryUserPermissionRepository::new(),
            )))
            .await;
            Runtime::register(UserPasswordPolicyRepository::new(Arc::new(
                InMemoryUserPasswordPolicyRepository::new(),
            )))
            .await;
            Runtime::register(UserPasswordRepository::new(Arc::new(
                InMemoryUserPasswordRepository::new(),
            )))
            .await;
            Ok(())
        })
    }
}
