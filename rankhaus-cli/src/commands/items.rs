use anyhow::{bail, Result};
use crate::state::AppState;
use crate::ItemsCommands;

pub fn execute(command: ItemsCommands, state: Option<&mut AppState>) -> Result<()> {
    // Check if list is loaded
    let has_list = state.as_ref().map(|s| s.has_list()).unwrap_or(false);
    if !has_list {
        bail!("No list loaded. Use 'init <name>' or 'load <file>' first.");
    }
    
    match command {
        ItemsCommands::List => {
            list(state)
        }
        ItemsCommands::Add => {
            add(state)
        }
        ItemsCommands::Remove => {
            remove(state)
        }
        ItemsCommands::Edit { identifier, new_value } => {
            edit(state, identifier, new_value)
        }
    }
}

fn list(_state: Option<&mut AppState>) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn add(_state: Option<&mut AppState>) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn remove(_state: Option<&mut AppState>) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn edit(_state: Option<&mut AppState>, _identifier: String, _new_value: String) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}
