use anyhow::{bail, Result};
use crate::state::AppState;
use crate::StrategyCommands;

pub fn execute(command: StrategyCommands, _state: Option<&mut AppState>) -> Result<()> {
    match command {
        StrategyCommands::List => {
            list()
        }
        StrategyCommands::Select { strategy } => {
            select(strategy)
        }
    }
}

fn list() -> Result<()> {
    println!("Available strategies:");
    println!("  merge      - Merge sort (pairwise comparison)");
    #[cfg(feature = "elo")]
    println!("  elo        - Elo rating system");
    #[cfg(feature = "tournament")]
    println!("  tournament - Tournament/knockout");
    #[cfg(feature = "condorcet")]
    println!("  condorcet  - Condorcet method");
    #[cfg(feature = "active")]
    println!("  active     - Active learning");
    #[cfg(feature = "btm")]
    println!("  btm        - Bradley-Terry model");
    Ok(())
}

fn select(_strategy: String) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}
