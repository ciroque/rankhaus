use crate::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user who can create rankings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub username: String,
    pub display_name: String,
    pub created: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

impl User {
    /// Create a new user with a generated ID
    pub fn new(username: String, display_name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(Some("u")),
            username: username.clone(),
            display_name: display_name.unwrap_or(username),
            created: now,
            last_active: now,
        }
    }
    
    /// Update the last active timestamp
    pub fn touch(&mut self) {
        self.last_active = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new("alice".to_string(), None);
        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "alice");
        assert!(user.id.as_str().starts_with("u"));
    }
    
    #[test]
    fn test_user_with_display_name() {
        let user = User::new("alice".to_string(), Some("Alice Smith".to_string()));
        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "Alice Smith");
    }
}
