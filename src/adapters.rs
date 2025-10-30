use std::{
    collections::HashSet,
    ffi::c_void,
    mem::zeroed,
    pin::Pin,
    sync::{Arc, OnceLock},
};

use tokio::sync::Mutex;

use crate::traits::adapter_loader_trait::AdapterLoaderTrait;

pub mod repository;

pub(crate) static ADAPTER_LIST: OnceLock<Arc<Mutex<HashSet<Arc<dyn AdapterLoaderTrait>>>>> =
    OnceLock::new();

#[repr(C)]
pub struct Adapter {
    _phantom: c_void,
}

#[unsafe(no_mangle)]
pub extern "C" fn get_adapter() -> Adapter {
    Adapter::get_instance()
}

impl Adapter {
    pub fn get_instance() -> Self {
        Self {
            _phantom: unsafe { zeroed() },
        }
    }
    /// Add adapter into the adapter list to be loaded
    /// If the adapter is already in the list, it will be replaced by the new one
    /// If the adapter is not yet in the list, it will be added
    pub fn insert(
        &self,
        adapter: Arc<dyn AdapterLoaderTrait>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            if let Some(list) = ADAPTER_LIST.get() {
                let mut list = list.lock().await;
                list.insert(adapter);
            } else {
                let mut hashset = HashSet::new();
                hashset.insert(adapter);
                ADAPTER_LIST.set(Arc::new(Mutex::new(hashset))).unwrap();
            }
        })
    }
}
