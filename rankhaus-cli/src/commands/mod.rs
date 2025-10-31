use anyhow::Result;
use crate::Commands;
use crate::state::AppState;

mod init;
mod load;
mod items;
mod users;
mod strategy;
mod sessions;
mod rank;
mod ranksets;

/// Execute command in direct mode (no state)
pub fn execute(command: Commands) -> Result<()> {
    match command {
        Commands::Ranksets { command } => {
            ranksets::execute(command, None)
        }
        Commands::Items { command } => {
            items::execute(command, None)
        }
        Commands::Users { command } => {
            users::execute(command, None)
        }
        Commands::Strategies { command } => {
            strategy::execute(command, None)
        }
        Commands::Rank => {
            rank::start(None)
        }
        Commands::Sessions { command } => {
            sessions::execute(command, None)
        }
    }
}

/// Execute command in REPL mode (with state)
pub fn execute_with_state(command: Commands, state: &mut AppState) -> Result<()> {
    match command {
        Commands::Ranksets { command } => {
            ranksets::execute(command, Some(state))
        }
        Commands::Items { command } => {
            items::execute(command, Some(state))
        }
        Commands::Users { command } => {
            users::execute(command, Some(state))
        }
        Commands::Strategies { command } => {
            strategy::execute(command, Some(state))
        }
        Commands::Rank => {
            rank::start(Some(state))
        }
        Commands::Sessions { command } => {
            sessions::execute(command, Some(state))
        }
    }
}
