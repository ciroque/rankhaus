use anyhow::{bail, Result};
use crate::state::AppState;
use crate::UsersCommands;

pub fn execute(command: UsersCommands, state: Option<&mut AppState>) -> Result<()> {
    // Check if list is loaded
    let has_list = state.as_ref().map(|s| s.has_list()).unwrap_or(false);
    if !has_list {
        bail!("No list loaded. Use 'init <name>' or 'load <file>' first.");
    }
    
    match command {
        UsersCommands::List => {
            list(state)
        }
        UsersCommands::Add { username, display_name } => {
            add(state, username, display_name)
        }
        UsersCommands::Remove { identifier, cascade } => {
            remove(state, identifier, cascade)
        }
        UsersCommands::Edit { identifier, new_display_name } => {
            edit(state, identifier, new_display_name)
        }
        UsersCommands::Select { identifier } => {
            select(state, identifier)
        }
    }
}

fn list(_state: Option<&mut AppState>) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn add(_state: Option<&mut AppState>, _username: String, _display_name: Option<String>) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn remove(_state: Option<&mut AppState>, _identifier: String, _cascade: bool) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn edit(_state: Option<&mut AppState>, _identifier: String, _new_display_name: String) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn select(_state: Option<&mut AppState>, _identifier: String) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}
