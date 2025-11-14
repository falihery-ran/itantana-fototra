use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Authentication required")]
    NotAuthenticated,
    #[error("Operation forbiden")]
    NotAuthorized,
}
