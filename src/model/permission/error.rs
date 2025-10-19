use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermissionError {
    #[error("The permission {name} does not exists")]
    PermissionNotExists { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
