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

/// Execute command in direct mode (no state)
pub fn execute(command: Commands) -> Result<()> {
    match command {
        Commands::Init { name, user, display_name, description, author } => {
            init::execute(name, user, display_name, description, author, None)
        }
        Commands::Load { file } => {
            load::execute(file, None)
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
            rank::execute(None)
        }
        Commands::Sessions { command } => {
            sessions::execute(command, None)
        }
    }
}

/// Execute command in REPL mode (with state)
pub fn execute_with_state(command: Commands, state: &mut AppState) -> Result<()> {
    match command {
        Commands::Init { name, user, display_name, description, author } => {
            init::execute(name, user, display_name, description, author, Some(state))
        }
        Commands::Load { file } => {
            load::execute(file, Some(state))
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
            rank::execute(Some(state))
        }
        Commands::Sessions { command } => {
            sessions::execute(command, Some(state))
        }
    }
}
