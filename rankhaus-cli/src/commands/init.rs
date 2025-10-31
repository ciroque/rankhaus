use crate::state::AppState;
use anyhow::{Context, Result};
use rankhaus::{RankSet, User};
use std::path::PathBuf;

pub fn execute(
    name: String,
    user: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    author: Option<String>,
    state: Option<&mut AppState>,
) -> Result<()> {
    // Use provided username or default to "<default>"
    let username = user.unwrap_or_else(|| "<default>".to_string());

    // Use provided display name or default to username
    let user_display_name = display_name.or_else(|| Some(username.clone()));

    // Use provided description or none
    let list_description = description;

    // Create the list
    let list_author = author.unwrap_or_else(|| username.clone());
    let mut list = RankSet::new(name.clone(), list_author, list_description);

    // Add the initial user (first user is always default)
    let mut user = User::new(username.clone(), user_display_name);
    user.default = true;
    let user_id = user.id.to_string();
    list.add_user(user)?;

    // Set the file path to ranksets directory
    let filename = format!("{}.rankset", name);
    let filepath = PathBuf::from("ranksets").join(&filename);

    // Create ranksets directory if it doesn't exist
    if let Some(parent) = filepath.parent() {
        std::fs::create_dir_all(parent).context("Failed to create ranksets directory")?;
    }

    list.file_path = Some(filepath.clone());

    // Save the list
    list.save()?;

    println!("✓ Created: {}", filepath.display());
    println!("✓ Active user: {} ({})", username, user_id);

    // If in REPL mode, load the list into state
    if let Some(state) = state {
        state.rankset = Some(list);
        state.active_user_id = Some(user_id.into());
        println!("✓ RankSet loaded into session");
    } else {
        println!("\nNext steps:");
        println!(
            "  rankhaus ranksets load {}  # Load the rankset",
            filepath.display()
        );
        println!("  rankhaus items add              # Add items to rank");
    }

    Ok(())
}
