use crate::state::AppState;
use crate::StrategyCommands;
use anyhow::{bail, Result};

pub fn execute(command: StrategyCommands, state: Option<&mut AppState>) -> Result<()> {
    match command {
        StrategyCommands::List => list(state),
        StrategyCommands::Select { strategy } => select(strategy, state),
    }
}

fn list(state: Option<&mut AppState>) -> Result<()> {
    let active_strategy = state.map(|s| s.active_strategy.as_str());
    
    println!("Available strategies:");
    
    let marker = if active_strategy == Some("merge") { "*" } else { " " };
    println!("{} merge      - Merge sort (pairwise comparison)", marker);
    
    #[cfg(feature = "quicksort")]
    {
        let marker = if active_strategy == Some("quicksort") { "*" } else { " " };
        println!("{} quicksort  - Quick sort (pivot-based partitioning)", marker);
    }
    
    #[cfg(feature = "elo")]
    {
        let marker = if active_strategy == Some("elo") { "*" } else { " " };
        println!("{} elo        - Elo rating system", marker);
    }
    
    #[cfg(feature = "tournament")]
    {
        let marker = if active_strategy == Some("tournament") { "*" } else { " " };
        println!("{} tournament - Tournament/knockout", marker);
    }
    
    #[cfg(feature = "condorcet")]
    {
        let marker = if active_strategy == Some("condorcet") { "*" } else { " " };
        println!("{} condorcet  - Condorcet method", marker);
    }
    
    #[cfg(feature = "active")]
    {
        let marker = if active_strategy == Some("active") { "*" } else { " " };
        println!("{} active     - Active learning", marker);
    }
    
    #[cfg(feature = "btm")]
    {
        let marker = if active_strategy == Some("btm") { "*" } else { " " };
        println!("{} btm        - Bradley-Terry model", marker);
    }
    
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
