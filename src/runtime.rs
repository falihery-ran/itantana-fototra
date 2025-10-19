use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use tokio::sync::RwLock;

use crate::{
    adapters::ADAPTER_LIST,
    configuration::load_configuration,
    repository::{
        permission_repository::PermissionRepository,
        user_internet_repository::UserInternetRepository,
        user_password_policy_repository::UserPasswordPolicyRepository,
        user_password_repository::UserPasswordRepository,
        user_permission_repository::UserPermissionRepository, user_repository::UserRepository,
    },
};

type ServiceRegistry = HashMap<TypeId, Box<dyn Any + Send + Sync>>;
static REGISTRY: OnceLock<RwLock<ServiceRegistry>> = OnceLock::new();

pub struct Runtime;

impl Runtime {
    /// Initialize the runtime (call once at startup)
    pub async fn init() -> Result<(), &'static str> {
        REGISTRY
            .set(RwLock::new(HashMap::new()))
            .map_err(|_| "Runtime already initialized")?;
        Runtime::register(load_configuration()).await;
        let adapter_list = ADAPTER_LIST.get().unwrap().lock().await;
        for adapter in adapter_list.iter() {
            adapter.load().await.unwrap();
        }
        for adapter in adapter_list.iter() {
            adapter.initialize().await.unwrap();
        }

        Self::verify_initialized().await
    }

    async fn verify_initialized() -> Result<(), &'static str> {
        Runtime::get::<PermissionRepository>()
            .await
            .ok_or("PermissionRepository not registered")?;
        Runtime::get::<UserRepository>()
            .await
            .ok_or("UserRepository not registered")?;
        Runtime::get::<UserInternetRepository>()
            .await
            .ok_or("UserInternetRepository not registered")?;
        Runtime::get::<UserPasswordRepository>()
            .await
            .ok_or("UserPasswordRepository not registered")?;
        Runtime::get::<UserPasswordPolicyRepository>()
            .await
            .ok_or("UserPasswordPolicyRepository not registered")?;
        Runtime::get::<UserPermissionRepository>()
            .await
            .ok_or("UserPermissionRepository not registered")?;
        Ok(())
    }

    /// Register a service
    pub async fn register<T: Send + Sync + 'static>(service: T) {
        let registry = REGISTRY.get().expect("Runtime not initialized");
        let mut map = registry.write().await;
        map.insert(TypeId::of::<T>(), Box::new(Arc::new(service)));
    }

    /// Get a service
    pub async fn get<T: Send + Sync + 'static>() -> Option<Arc<T>> {
        let registry = REGISTRY.get().expect("Runtime not initialized");
        let map = registry.read().await;
        map.get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<Arc<T>>())
            .cloned()
    }

    /// Check if runtime is initialized
    pub fn is_initialized() -> bool {
        REGISTRY.get().is_some()
    }
}

// Usage:
// #[tokio::main]
// async fn main() {
//     Runtime::init().unwrap();

//     Runtime::register("Hello World".to_string()).await;

//     let service: Option<Arc<String>> = Runtime::get().await;
//     println!("{:?}", service);
// }
