use anyhow::{Context, Result};
use crate::state::AppState;
use rankhaus::List;

pub fn execute(file: String, state: Option<&mut AppState>) -> Result<()> {
    let list = List::load(&file)
        .context(format!("Failed to load list from {}", file))?;
    
    println!("✓ Loaded: {}", list.meta.name);
    println!("  Items: {}", list.items.len());
    println!("  Users: {}", list.users.len());
    println!("  Rankings: {}", list.rankings.len());
    
    // Store in state if in REPL mode
    if let Some(state) = state {
        state.list = Some(list);
        println!("✓ List loaded into session");
    }
    
    Ok(())
}
