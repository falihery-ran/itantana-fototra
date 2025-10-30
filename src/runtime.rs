use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ffi::c_void,
    mem::zeroed,
    sync::{Arc, LazyLock, OnceLock},
};

use futures::future::join_all;
use tokio::{
    sync::{Mutex, RwLock},
    task::JoinHandle,
};

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

type ServiceRegistry = HashMap<TypeId, Arc<dyn Any + Send + Sync>>;
static REGISTRY: OnceLock<Arc<RwLock<ServiceRegistry>>> = OnceLock::new();
static INIT_LIST: LazyLock<Arc<Mutex<Option<Vec<JoinHandle<Result<(), anyhow::Error>>>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

#[repr(C)]
pub struct Runtime {
    _phantom: c_void,
}

#[unsafe(no_mangle)]
pub extern "C" fn get_runtime() -> Runtime {
    Runtime::get_instance()
}

impl Runtime {
    pub fn get_instance() -> Self {
        Self {
            _phantom: unsafe { zeroed() },
        }
    }

    /// Initialize the runtime (call once at startup)
    pub async fn init(&self) -> Result<(), &'static str> {
        if REGISTRY.get().is_none() {
            REGISTRY
                .set(Arc::new(RwLock::new(HashMap::new())))
                .map_err(|_| "Runtime already initialized")?;
            self.register(load_configuration()).await;
            let adapter_list = ADAPTER_LIST.get().unwrap().lock().await;
            let mut load_list = Vec::new();
            let mut init_list = INIT_LIST.lock().await;
            let mut list = Vec::new();
            for adapter in adapter_list.iter() {
                let adapter_cloned = adapter.clone();
                load_list.push(tokio::spawn(async move { adapter_cloned.load().await }));
                let adapter_cloned = adapter.clone();
                list.push(tokio::spawn(
                    async move { adapter_cloned.initialize().await },
                ));
            }
            *init_list = Some(list);
            let load_result = join_all(load_list).await;
            let _ = load_result.into_iter().map(|l| l.unwrap());

            self.verify_initialized().await?;
        }
        Ok(())
    }

    async fn verify_initialized(&self) -> Result<(), &'static str> {
        self.get::<PermissionRepository>()
            .await
            .ok_or("PermissionRepository not registered")?;
        self.get::<UserRepository>()
            .await
            .ok_or("UserRepository not registered")?;
        self.get::<UserInternetRepository>()
            .await
            .ok_or("UserInternetRepository not registered")?;
        self.get::<UserPasswordRepository>()
            .await
            .ok_or("UserPasswordRepository not registered")?;
        self.get::<UserPasswordPolicyRepository>()
            .await
            .ok_or("UserPasswordPolicyRepository not registered")?;
        self.get::<UserPermissionRepository>()
            .await
            .ok_or("UserPermissionRepository not registered")?;
        Ok(())
    }

    /// Register a service
    pub async fn register<T: Send + Sync + 'static>(&self, service: T) {
        let registry = REGISTRY.get().expect("Runtime not initialized");
        let mut map = registry.write().await;
        map.insert(TypeId::of::<T>(), Arc::new(service));
    }

    /// Get a service
    pub async fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let registry = REGISTRY.get().expect("Runtime not initialized");
        let map = registry.read().await;
        map.get(&TypeId::of::<T>())
            .and_then(|any| any.clone().downcast::<T>().ok())
    }

    /// Check if runtime is initialized
    pub fn is_initialized(&self) -> bool {
        REGISTRY.get().is_some()
    }

    pub async fn end(&self) {
        let mut init_list = INIT_LIST.lock().await;
        let list = init_list.take().unwrap();
        for i in list {
            let _ = i.await.unwrap();
        }
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
