use anyhow::{bail, Result};
use crate::state::AppState;
use crate::SessionsCommands;

pub fn execute(command: SessionsCommands, state: Option<&mut AppState>) -> Result<()> {
    // Check if list is loaded
    let has_list = state.as_ref().map(|s| s.has_list()).unwrap_or(false);
    if !has_list {
        bail!("No list loaded. Use 'init <name>' or 'load <file>' first.");
    }
    
    match command {
        SessionsCommands::List => {
            list(state)
        }
        SessionsCommands::Show { session_id } => {
            show(state, session_id)
        }
        SessionsCommands::Delete { session_id } => {
            delete(state, session_id)
        }
    }
}

fn list(_state: Option<&mut AppState>) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn show(_state: Option<&mut AppState>, _session_id: String) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}

fn delete(_state: Option<&mut AppState>, _session_id: String) -> Result<()> {
    // TODO: Implement
    bail!("Not yet implemented");
}
