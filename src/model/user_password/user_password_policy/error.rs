use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserPasswordPolicyError {
    #[error("The policy {name} does not exists")]
    PolicyNotExists { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
