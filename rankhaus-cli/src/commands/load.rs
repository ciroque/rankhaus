use crate::state::AppState;
use anyhow::{Context, Result};
use rankhaus::RankSet;
use std::path::{Path, PathBuf};

pub fn execute(file: String, state: Option<&mut AppState>) -> Result<()> {
    // Resolve the file path
    let resolved_path = resolve_rankset_path(&file);

    let list = RankSet::load(&resolved_path).context(format!(
        "Failed to load list from {}",
        resolved_path.display()
    ))?;

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

/// Resolves a rankset file path by:
/// 1. If the path exists as-is, use it
/// 2. Otherwise, prepend "./ranksets/" and append ".rankset" extension if needed
fn resolve_rankset_path(file: &str) -> PathBuf {
    let path = Path::new(file);

    // If the path exists as-is, use it
    if path.exists() {
        return path.to_path_buf();
    }

    // Otherwise, try to resolve it in the ranksets directory
    let mut resolved = PathBuf::from("ranksets");
    resolved.push(file);

    // Add .rankset extension if not present
    if resolved.extension().is_none() {
        resolved.set_extension("rankset");
    }

    resolved
}
