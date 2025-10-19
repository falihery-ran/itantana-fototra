use std::{str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};

// Basic password: at least 8 characters, contains letter and number
static BASIC_PASSWORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?=.*[A-Za-z])(?=.*\d).{6,}$").unwrap());

// Medium password: 6+ chars, letter and number
static MEDIUM_PASSWORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap());

// Strong password: 8+ chars, uppercase, lowercase, number, special char
static STRONG_PASSWORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap()
});

// Very strong password: 12+ chars, all character types, no common patterns
static VERY_STRONG_PASSWORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&#+\-_=])[A-Za-z\d@$!%*?&#+\-_=]{12,}$")
        .unwrap()
});

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PasswordLevel {
    #[default]
    Basic,
    Medium,
    Strong,
    VeryStrong,
}

impl FromStr for PasswordLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" => Ok(Self::Basic),
            "medium" => Ok(Self::Medium),
            "strong" => Ok(Self::Strong),
            "very_strong" => Ok(Self::VeryStrong),
            _ => Err(
                "Password level should be one of: basic, medium, strong, very_strong".to_string(),
            ),
        }
    }
}

impl PasswordLevel {
    pub fn get_requirement(&self) -> &str {
        match self {
            PasswordLevel::Basic => "At least 6 characters with letters and numbers",
            PasswordLevel::Medium => "At least 8 characters with letters and numbers",
            PasswordLevel::Strong => {
                "At least 8 characters with uppercase, lowercase, numbers, and special characters"
            }
            PasswordLevel::VeryStrong => {
                "At least 12 characters with uppercase, lowercase, numbers, and special characters"
            }
        }
    }

    pub fn validate(&self, password: &str) -> bool {
        match self {
            PasswordLevel::Basic => BASIC_PASSWORD.is_match(password),
            PasswordLevel::Medium => MEDIUM_PASSWORD.is_match(password),
            PasswordLevel::Strong => STRONG_PASSWORD.is_match(password),
            PasswordLevel::VeryStrong => VERY_STRONG_PASSWORD.is_match(password),
        }
    }
}
