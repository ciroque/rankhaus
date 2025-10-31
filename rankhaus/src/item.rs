use crate::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An item to be ranked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip)]
    pub id: Id,
    pub value: String,
    pub created: DateTime<Utc>,
}

impl Item {
    /// Create a new item with a generated ID
    pub fn new(value: String) -> Self {
        Self {
            id: Id::new(None),
            value,
            created: Utc::now(),
        }
    }

    /// Create an item with a specific ID (for deserialization)
    pub fn with_id(id: Id, value: String, created: DateTime<Utc>) -> Self {
        Self { id, value, created }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new("test".to_string());
        assert_eq!(item.value, "test");
        assert_eq!(item.id.as_str().len(), 7);
    }
}
