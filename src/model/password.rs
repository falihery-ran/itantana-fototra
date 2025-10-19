pub mod password_level;

use anyhow::anyhow;
use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{Encoding, PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use serde::Serialize;
use thiserror::Error;

use crate::model::password::password_level::PasswordLevel;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(password: &str, password_level: &PasswordLevel) -> Result<Self, PasswordError> {
        if password_level.validate(password) {
            Ok(Self(password.to_string()))
        } else {
            Err(PasswordError::InvalidPassword(
                password_level.get_requirement().to_string(),
            ))
        }
    }

    pub fn hash(&self) -> Result<String, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(self.0.as_bytes(), &salt)
            .map_err(|e| PasswordError::Unknown(anyhow!(e.to_string())))?
            .to_string())
    }

    pub fn verify(password: &str, password_hash: &str) -> Result<(), PasswordError> {
        if let Ok(hash) = PasswordHash::parse(password_hash, Encoding::default()) {
            if Argon2::default()
                .verify_password(password.as_bytes(), &hash)
                .is_ok()
            {
                return Ok(());
            }
        }
        Err(PasswordError::IncorrectPassword)
    }
}

#[derive(Debug, Error)]
pub enum PasswordError {
    // when updating password
    #[error("Password invalid: {0}")]
    InvalidPassword(String),
    // when checking password
    #[error("Password incorrect")]
    IncorrectPassword,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
