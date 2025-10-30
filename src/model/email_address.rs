use std::{fmt::Display, ops::Deref, sync::LazyLock};

use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap());

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "&str")]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self, EmailAddressError> {
        let trimed = email.trim();
        Self::validate_email(trimed).map(|_| Self(trimed.to_string()))
    }

    fn validate_email(email: &str) -> Result<(), EmailAddressError> {
        if EMAIL_REGEX.is_match(email).map_err(|e| EmailAddressError {
            invalid_email: e.to_string(),
        })? {
            return Ok(());
        }
        Err(EmailAddressError {
            invalid_email: email.to_string(),
        })
    }
}

impl Deref for EmailAddress {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = EmailAddressError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Error)]
#[error("{invalid_email} is not a valid email address")]
pub struct EmailAddressError {
    pub invalid_email: String,
}
