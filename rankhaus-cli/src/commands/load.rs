use crate::state::AppState;
use anyhow::{Context, Result};
use rankhaus::RankSet;

pub fn execute(file: String, state: Option<&mut AppState>) -> Result<()> {
    let list = RankSet::load(&file).context(format!("Failed to load list from {}", file))?;

    println!("✓ Loaded: {}", list.meta.name);
    println!("  Items: {}", list.items.len());
    println!("  Users: {}", list.users.len());
    println!("  Rankings: {}", list.rankings.len());

    // Store in state if in REPL mode
    if let Some(state) = state {
        state.rankset = Some(list);
        println!("✓ RankSet loaded into session");
    }

    Ok(())
}
