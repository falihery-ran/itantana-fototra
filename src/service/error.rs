use std::{any::Any, sync::Arc};

#[derive(Debug)]
pub struct ServiceError {
    inner: Arc<dyn Any + Sync + Send>,
}

impl ServiceError {
    pub fn new<T: Send + Sync + 'static>(error: T) -> Self {
        Self {
            inner: Arc::new(error),
        }
    }
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.inner.downcast_ref::<Arc<T>>().cloned()
    }
}
