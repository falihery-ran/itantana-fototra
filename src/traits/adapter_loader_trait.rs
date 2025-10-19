use std::{fmt::Debug, hash::Hash, pin::Pin};

use crate::traits::initialize_trait::InitializeTrait;

pub trait AdapterLoaderTrait: InitializeTrait + Debug + Send + Sync {
    fn name(&self) -> &str;

    /// Add adapter into the runtime
    /// Adapter should be loaded before initialized
    /// call load before initialiaze
    fn load(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;
}

impl Hash for dyn AdapterLoaderTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for dyn AdapterLoaderTrait {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn AdapterLoaderTrait {}
