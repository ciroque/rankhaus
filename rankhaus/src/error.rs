use thiserror::Error;

/// Result type alias for rankhaus operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for rankhaus operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Strategy not found: {0}")]
    StrategyNotFound(String),
    
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    
    #[error("Duplicate item: {0}")]
    DuplicateItem(String),
    
    #[error("Duplicate user: {0}")]
    DuplicateUser(String),
    
    #[error("No list loaded")]
    NoListLoaded,
    
    #[error("No active user")]
    NoActiveUser,
    
    #[error("Cannot remove user with existing rankings (use --cascade to force)")]
    UserHasRankings,
    
    #[error("{0}")]
    Other(String),
}
