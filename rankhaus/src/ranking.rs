use crate::{Id, Session};
use serde::{Deserialize, Serialize};

pub use crate::strategy::RankResult;

/// A complete ranking with results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranking {
    pub user_id: Id,
    pub strategy: String,
    pub session: Session,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RankResult>,
}

impl Ranking {
    /// Create a new ranking
    pub fn new(user_id: Id, strategy: String) -> Self {
        Self {
            user_id,
            strategy,
            session: Session::new(),
            result: None,
        }
    }
    
    /// Check if this ranking is complete
    pub fn is_complete(&self) -> bool {
        self.result.is_some()
    }
}
