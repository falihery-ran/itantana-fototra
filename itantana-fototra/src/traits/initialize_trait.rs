use std::pin::Pin;

pub trait InitializeTrait: Sync + Send + 'static {
    fn initialize<'a>(&'a self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }
}
