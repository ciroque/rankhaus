//! Rankhaus - A flexible ranking library for interactive stack ranking
//!
//! This library provides core data structures and ranking strategies for
//! performing pairwise comparisons and generating ranked orderings.

pub mod error;
pub mod id;
pub mod item;
pub mod list;
pub mod ranking;
pub mod session;
pub mod strategy;
pub mod user;

// Re-export commonly used types
pub use error::{Error, Result};
pub use id::Id;
pub use item::Item;
pub use list::List;
pub use ranking::{RankResult, Ranking};
pub use session::Session;
pub use strategy::RankStrategy;
pub use user::User;
