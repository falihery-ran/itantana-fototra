use std::pin::Pin;

use crate::security::error::SecurityError;

pub trait AuthorizationTrait: Sync + Send + 'static {
    fn authorize<'a>(
        &'a self,
        permission: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), SecurityError>> + Send + 'a>>;
}
