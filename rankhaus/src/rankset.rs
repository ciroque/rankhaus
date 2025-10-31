use crate::{Error, Item, Ranking, Result, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Metadata about a ranking set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankSetMeta {
    pub name: String,
    #[serde(rename = "type")]
    pub list_type: String,
    pub author: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

/// A complete ranking set with items, users, and rankings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankSet {
    pub meta: RankSetMeta,
    pub users: HashMap<String, User>,
    pub items: HashMap<String, Item>,
    pub rankings: Vec<Ranking>,
    
    #[serde(skip)]
    pub file_path: Option<PathBuf>,
}

impl RankSet {
    /// Create a new empty rank set
    pub fn new(name: String, author: String, description: String) -> Self {
        Self {
            meta: RankSetMeta {
                name,
                list_type: "rankset".to_string(),
                author,
                description,
                created: Utc::now(),
            },
            users: HashMap::new(),
            items: HashMap::new(),
            rankings: Vec::new(),
            file_path: None,
        }
    }
    
    /// Load a rank set from a JSON file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        let mut rankset: RankSet = serde_json::from_str(&content)?;
        rankset.file_path = Some(path.as_ref().to_path_buf());
        Ok(rankset)
    }
    
    /// Save the rank set to its file path
    pub fn save(&self) -> Result<()> {
        if let Some(path) = &self.file_path {
            self.save_to(path)
        } else {
            Err(Error::Other("No file path set".to_string()))
        }
    }
    
    /// Save the rank set to a specific path
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Add a user to the rank set
    pub fn add_user(&mut self, user: User) -> Result<()> {
        let id = user.id.to_string();
        if self.users.contains_key(&id) {
            return Err(Error::DuplicateUser(user.username));
        }
        self.users.insert(id, user);
        Ok(())
    }
    
    /// Get a user by ID or username
    pub fn get_user(&self, identifier: &str) -> Result<&User> {
        // Try as ID first
        if let Some(user) = self.users.get(identifier) {
            return Ok(user);
        }
        
        // Try as username
        self.users
            .values()
            .find(|u| u.username == identifier)
            .ok_or_else(|| Error::UserNotFound(identifier.to_string()))
    }
    
    /// Get a mutable user by ID or username
    pub fn get_user_mut(&mut self, identifier: &str) -> Result<&mut User> {
        // Try as ID first
        if self.users.contains_key(identifier) {
            return Ok(self.users.get_mut(identifier).unwrap());
        }
        
        // Try as username
        let id = self.users
            .values()
            .find(|u| u.username == identifier)
            .map(|u| u.id.to_string())
            .ok_or_else(|| Error::UserNotFound(identifier.to_string()))?;
        
        Ok(self.users.get_mut(&id).unwrap())
    }
    
    /// Remove a user
    pub fn remove_user(&mut self, identifier: &str, cascade: bool) -> Result<()> {
        let user = self.get_user(identifier)?;
        let user_id = user.id.clone();
        
        // Check if user has rankings
        let has_rankings = self.rankings.iter().any(|r| r.user_id == user_id);
        if has_rankings && !cascade {
            return Err(Error::UserHasRankings);
        }
        
        // Remove rankings if cascade
        if cascade {
            self.rankings.retain(|r| r.user_id != user_id);
        }
        
        // Remove user
        self.users.remove(&user_id.to_string());
        Ok(())
    }
    
    /// Add an item to the rank set
    pub fn add_item(&mut self, item: Item) -> Result<()> {
        let id = item.id.to_string();
        if self.items.contains_key(&id) {
            return Err(Error::DuplicateItem(item.value));
        }
        self.items.insert(id, item);
        Ok(())
    }
    
    /// Get an item by ID or value
    pub fn get_item(&self, identifier: &str) -> Result<&Item> {
        // Try as ID first
        if let Some(item) = self.items.get(identifier) {
            return Ok(item);
        }
        
        // Try as value
        self.items
            .values()
            .find(|i| i.value == identifier)
            .ok_or_else(|| Error::ItemNotFound(identifier.to_string()))
    }
    
    /// Get a mutable item by ID or value
    pub fn get_item_mut(&mut self, identifier: &str) -> Result<&mut Item> {
        // Try as ID first
        if self.items.contains_key(identifier) {
            return Ok(self.items.get_mut(identifier).unwrap());
        }
        
        // Try as value
        let id = self.items
            .values()
            .find(|i| i.value == identifier)
            .map(|i| i.id.to_string())
            .ok_or_else(|| Error::ItemNotFound(identifier.to_string()))?;
        
        Ok(self.items.get_mut(&id).unwrap())
    }
    
    /// Remove an item
    pub fn remove_item(&mut self, identifier: &str) -> Result<()> {
        let item = self.get_item(identifier)?;
        let item_id = item.id.to_string();
        self.items.remove(&item_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rankset_creation() {
        let rankset = RankSet::new(
            "test".to_string(),
            "author".to_string(),
            "description".to_string(),
        );
        assert_eq!(rankset.meta.name, "test");
        assert_eq!(rankset.users.len(), 0);
        assert_eq!(rankset.items.len(), 0);
    }
    
    #[test]
    fn test_add_user() {
        let mut rankset = RankSet::new(
            "test".to_string(),
            "author".to_string(),
            "description".to_string(),
        );
        let user = User::new("alice".to_string(), None);
        rankset.add_user(user).unwrap();
        assert_eq!(rankset.users.len(), 1);
    }
    
    #[test]
    fn test_add_item() {
        let mut rankset = RankSet::new(
            "test".to_string(),
            "author".to_string(),
            "description".to_string(),
        );
        let item = Item::new("blue".to_string());
        rankset.add_item(item).unwrap();
        assert_eq!(rankset.items.len(), 1);
    }
    
    #[test]
    fn test_get_user_by_id() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let user = User::new("alice".to_string(), None);
        let user_id = user.id.to_string();
        rankset.add_user(user).unwrap();
        
        let found = rankset.get_user(&user_id).unwrap();
        assert_eq!(found.username, "alice");
    }
    
    #[test]
    fn test_get_user_by_username() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let user = User::new("alice".to_string(), None);
        rankset.add_user(user).unwrap();
        
        let found = rankset.get_user("alice").unwrap();
        assert_eq!(found.username, "alice");
    }
    
    #[test]
    fn test_get_user_not_found() {
        let rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let result = rankset.get_user("nonexistent");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_get_item_by_id() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let item = Item::new("blue".to_string());
        let item_id = item.id.to_string();
        rankset.add_item(item).unwrap();
        
        let found = rankset.get_item(&item_id).unwrap();
        assert_eq!(found.value, "blue");
    }
    
    #[test]
    fn test_get_item_by_value() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let item = Item::new("blue".to_string());
        rankset.add_item(item).unwrap();
        
        let found = rankset.get_item("blue").unwrap();
        assert_eq!(found.value, "blue");
    }
    
    #[test]
    fn test_get_item_not_found() {
        let rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let result = rankset.get_item("nonexistent");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_remove_item() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let item = Item::new("blue".to_string());
        rankset.add_item(item).unwrap();
        assert_eq!(rankset.items.len(), 1);
        
        rankset.remove_item("blue").unwrap();
        assert_eq!(rankset.items.len(), 0);
    }
    
    #[test]
    fn test_duplicate_item() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let item1 = Item::new("blue".to_string());
        let id = item1.id.clone();
        rankset.add_item(item1).unwrap();
        
        let item2 = Item::with_id(id, "blue".to_string(), Utc::now());
        let result = rankset.add_item(item2);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_duplicate_user() {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        let user1 = User::new("alice".to_string(), None);
        let id = user1.id.clone();
        rankset.add_user(user1).unwrap();
        
        let user2 = User::with_id(id, "alice".to_string(), None);
        let result = rankset.add_user(user2);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), "desc".to_string());
        rankset.add_item(Item::new("blue".to_string())).unwrap();
        rankset.add_user(User::new("alice".to_string(), None)).unwrap();
        
        let path = "test_save_load.rankset";
        rankset.file_path = Some(path.into());
        rankset.save().unwrap();
        
        let loaded = RankSet::load(path).unwrap();
        assert_eq!(loaded.meta.name, "test");
        assert_eq!(loaded.items.len(), 1);
        assert_eq!(loaded.users.len(), 1);
        
        fs::remove_file(path).unwrap();
    }
}
