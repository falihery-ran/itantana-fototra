use std::{pin::Pin, sync::Arc};

use crate::{security::error::SecurityError, traits::authorization_trait::AuthorizationTrait};

pub trait AuthenticationTrait: Sync + Send + 'static {
    fn authenticate<'a>(
        &'a self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Arc<dyn AuthorizationTrait + 'static>, SecurityError>>
                + Send
                + 'a,
        >,
    >;
}
