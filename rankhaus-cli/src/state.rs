use anyhow::Result;
use rankhaus::{List, Id};
use std::path::PathBuf;

/// Application state for REPL mode
pub struct AppState {
    pub list: Option<List>,
    pub active_user_id: Option<Id>,
    #[allow(dead_code)] // Will be used when strategy selection is implemented
    pub active_strategy: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            list: None,
            active_user_id: None,
            active_strategy: "merge".to_string(),
        }
    }
    
    #[allow(dead_code)] // Will be used for alternative load path
    pub fn load_list(&mut self, path: PathBuf) -> Result<()> {
        self.list = Some(List::load(path)?);
        Ok(())
    }
    
    pub fn has_list(&self) -> bool {
        self.list.is_some()
    }
    
    #[allow(dead_code)] // Will be used when implementing item/user commands
    pub fn get_list(&self) -> Option<&List> {
        self.list.as_ref()
    }
    
    #[allow(dead_code)] // Will be used when implementing item/user commands
    pub fn get_list_mut(&mut self) -> Option<&mut List> {
        self.list.as_mut()
    }
    
    pub fn save(&mut self) -> Result<()> {
        if let Some(list) = &self.list {
            list.save()?;
        }
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
