use crate::{Id, Item, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of a ranking operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankResult {
    /// Ordered list of item IDs (best to worst)
    pub order: Option<Vec<Id>>,
    /// Rating scores for each item
    pub ratings: Option<HashMap<Id, f64>>,
}

/// Trait for ranking strategies
pub trait RankStrategy: Send + Sync {
    /// Get the name of this strategy
    fn name(&self) -> &'static str;
    
    /// Perform a single comparison between two items
    /// Returns the ID of the preferred item
    fn compare(&mut self, a: &Item, b: &Item, winner_id: &Id) -> Result<()>;
    
    /// Complete the ranking and return results
    fn finalize(&mut self) -> Result<RankResult>;
    
    /// Serialize the current state for persistence
    fn serialize_state(&self) -> Result<serde_json::Value>;
    
    /// Deserialize and restore state
    fn deserialize_state(&mut self, state: serde_json::Value) -> Result<()>;
    
    /// Get the next pair of items to compare, if any
    fn next_comparison(&self) -> Option<(Id, Id)>;
    
    /// Check if ranking is complete
    fn is_complete(&self) -> bool;
}

#[cfg(feature = "merge")]
pub mod merge;

#[cfg(feature = "elo")]
pub mod elo;

#[cfg(feature = "tournament")]
pub mod tournament;

#[cfg(feature = "condorcet")]
pub mod condorcet;

#[cfg(feature = "active")]
pub mod active;

#[cfg(feature = "btm")]
pub mod btm;
