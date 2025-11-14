use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "&str")]
pub struct Name(String);

impl Name {
    pub fn new(raw: &str) -> Result<Self, NameError> {
        let raw_trimed = raw.trim();
        Ok(Self(Self::validate_name(raw_trimed)?.to_string()))
    }

    fn validate_name(raw: &str) -> Result<&str, NameError> {
        if let Some(first_char) = raw.chars().next() {
            if first_char.is_uppercase() {
                return Ok(raw);
            }
        }
        Err(NameError::InvalidName(raw.to_string()))
    }
}

impl TryFrom<&str> for Name {
    type Error = NameError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Error)]

pub enum NameError {
    #[error("{0} is not a valid name. Name should not be empty and must begin with capital.")]
    InvalidName(String),
}
