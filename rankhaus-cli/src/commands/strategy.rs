use crate::state::AppState;
use crate::StrategyCommands;
use anyhow::{bail, Result};

pub fn execute(command: StrategyCommands, state: Option<&mut AppState>) -> Result<()> {
    match command {
        StrategyCommands::List => list(),
        StrategyCommands::Select { strategy } => select(strategy, state),
    }
}

fn list() -> Result<()> {
    println!("Available strategies:");
    println!("  merge      - Merge sort (pairwise comparison)");
    #[cfg(feature = "quicksort")]
    println!("  quicksort  - Quick sort (pivot-based partitioning)");
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

fn select(strategy: String, state: Option<&mut AppState>) -> Result<()> {
    let app_state = state.ok_or_else(|| anyhow::anyhow!("No state available"))?;
    
    // Validate strategy is available
    let valid_strategies = get_available_strategies();
    
    if !valid_strategies.contains(&strategy.as_str()) {
        bail!(
            "Unknown strategy: '{}'. Available strategies: {}",
            strategy,
            valid_strategies.join(", ")
        );
    }
    
    app_state.active_strategy = strategy.clone();
    println!("✓ Selected strategy: {}", strategy);
    
    Ok(())
}

fn get_available_strategies() -> Vec<&'static str> {
    #[allow(unused_mut)]
    let mut strategies = vec!["merge"];
    
    #[cfg(feature = "quicksort")]
    strategies.push("quicksort");
    
    #[cfg(feature = "elo")]
    strategies.push("elo");
    
    #[cfg(feature = "tournament")]
    strategies.push("tournament");
    
    #[cfg(feature = "condorcet")]
    strategies.push("condorcet");
    
    #[cfg(feature = "active")]
    strategies.push("active");
    
    #[cfg(feature = "btm")]
    strategies.push("btm");
    
    strategies
}
