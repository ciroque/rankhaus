use anyhow::Result;
use rankhaus::{RankSet, Id};
use std::path::PathBuf;

/// Application state for REPL mode
pub struct AppState {
    pub rankset: Option<RankSet>,
    pub active_user_id: Option<Id>,
    #[allow(dead_code)] // Will be used when strategy selection is implemented
    pub active_strategy: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rankset: None,
            active_user_id: None,
            active_strategy: "merge".to_string(),
        }
    }
    
    #[allow(dead_code)] // Will be used for alternative load path
    pub fn load_rankset(&mut self, path: PathBuf) -> Result<()> {
        self.rankset = Some(RankSet::load(path)?);
        Ok(())
    }
    
    pub fn has_rankset(&self) -> bool {
        self.rankset.is_some()
    }
    
    #[allow(dead_code)] // Will be used when implementing item/user commands
    pub fn get_rankset(&self) -> Option<&RankSet> {
        self.rankset.as_ref()
    }
    
    #[allow(dead_code)] // Will be used when implementing item/user commands
    pub fn get_rankset_mut(&mut self) -> Option<&mut RankSet> {
        self.rankset.as_mut()
    }
    
    pub fn save(&mut self) -> Result<()> {
        if let Some(rankset) = &self.rankset {
            rankset.save()?;
        }
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
