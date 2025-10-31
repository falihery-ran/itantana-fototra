use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    ffi::c_void,
    mem::zeroed,
    sync::{Arc, OnceLock},
};

use tokio::sync::{Mutex, RwLock};

use crate::{configuration::load_configuration, traits::adapter_loader_trait::AdapterLoaderTrait};

type ServiceRegistry = HashMap<TypeId, Arc<dyn Any + Send + Sync>>;
static REGISTRY: OnceLock<Arc<RwLock<ServiceRegistry>>> = OnceLock::new();
static ADAPTER_LIST: OnceLock<Arc<Mutex<HashSet<Arc<dyn AdapterLoaderTrait>>>>> = OnceLock::new();

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
        }
        Ok(())
    }

    pub async fn add_adapter(&self, adapter: Arc<dyn AdapterLoaderTrait>) {
        let adapter = adapter.clone();
        if let Some(list) = ADAPTER_LIST.get() {
            let mut list = list.lock().await;
            list.insert(adapter.clone());
        } else {
            let mut hashset = HashSet::new();
            hashset.insert(adapter.clone());
            ADAPTER_LIST.set(Arc::new(Mutex::new(hashset))).unwrap();
        }
        adapter.load().await.unwrap();
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
}

// Usage:
// #[tokio::main]
// async fn main() {
//     Runtime::init().unwrap();

//     Runtime::register("Hello World".to_string()).await;

//     let service: Option<Arc<String>> = Runtime::get().await;
//     println!("{:?}", service);
// }
