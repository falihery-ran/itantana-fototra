use std::{collections::HashSet, pin::Pin, sync::OnceLock};

use tokio::sync::Mutex;

use crate::traits::adapter_loader_trait::AdapterLoaderTrait;

pub mod repository;

pub(crate) static ADAPTER_LIST: OnceLock<Mutex<HashSet<Box<dyn AdapterLoaderTrait>>>> =
    OnceLock::new();

pub struct Adapter;

impl Adapter {
    /// Add adapter into the adapter list to be loaded
    /// If the adapter is already in the list, it will be replaced by the new one
    /// If the adapter is not yet in the list, it will be added
    pub fn insert(
        adapter: Box<dyn AdapterLoaderTrait>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            if let Some(list) = ADAPTER_LIST.get() {
                let mut list = list.lock().await;
                list.insert(adapter);
            } else {
                let mut hashset = HashSet::new();
                hashset.insert(adapter);
                ADAPTER_LIST.set(Mutex::new(hashset)).unwrap();
            }
        })
    }
}
