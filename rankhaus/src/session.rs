use crate::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a ranking session
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    InProgress,
    Completed,
    Abandoned,
}

/// Metadata about a ranking session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<DateTime<Utc>>,
    pub status: SessionStatus,
}

impl SessionInfo {
    /// Create a new session
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created: now,
            last_updated: now,
            completed: None,
            status: SessionStatus::InProgress,
        }
    }
    
    /// Mark the session as updated
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
    
    /// Mark the session as completed
    pub fn complete(&mut self) {
        let now = Utc::now();
        self.completed = Some(now);
        self.last_updated = now;
        self.status = SessionStatus::Completed;
    }
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// A single comparison made during ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub a: Id,
    pub b: Id,
    pub winner: Id,
    pub timestamp: DateTime<Utc>,
}

/// A complete ranking session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(flatten)]
    pub info: SessionInfo,
    pub comparisons: Vec<Comparison>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
}

impl Session {
    /// Create a new session
    pub fn new() -> Self {
        Self {
            info: SessionInfo::new(),
            comparisons: Vec::new(),
            state: None,
        }
    }
    
    /// Add a comparison to the session
    pub fn add_comparison(&mut self, a: Id, b: Id, winner: Id) {
        self.comparisons.push(Comparison {
            a,
            b,
            winner,
            timestamp: Utc::now(),
        });
        self.info.touch();
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
