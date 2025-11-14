use std::{any::{Any, TypeId}, collections::HashMap, sync::{Arc, RwLock}};

use lazy_static::lazy_static;

use crate::configuration::{load_configuration, Configuration};

lazy_static!{
    pub static ref SETTINGS: Arc<Configuration> = Arc::new(load_configuration());
pub static ref RUNTIME: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}