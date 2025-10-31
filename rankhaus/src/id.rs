use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A 7-character alphanumeric identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Id(String);

impl Id {
    /// Generate a new random ID with optional prefix
    pub fn new(prefix: Option<&str>) -> Self {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();

        let random: String = (0..7)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        if let Some(p) = prefix {
            Self(format!("{}{}", p, random))
        } else {
            Self(random)
        }
    }

    /// Create an ID from a string (for deserialization)
    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl FromStr for Id {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id = Id::new(None);
        assert_eq!(id.as_str().len(), 7);
    }

    #[test]
    fn test_id_with_prefix() {
        let id = Id::new(Some("u"));
        assert_eq!(id.as_str().len(), 8);
        assert!(id.as_str().starts_with("u"));
    }
}
