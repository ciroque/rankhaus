use anyhow::{Context, Result};
use crate::state::AppState;
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
    // Prompt for username if not provided
    let username = if let Some(u) = user {
        u
    } else {
        inquire::Text::new("Enter your username:")
            .prompt()
            .context("Failed to read username")?
    };
    
    // Prompt for display name if not provided
    let user_display_name = if let Some(d) = display_name {
        Some(d)
    } else {
        let prompt = inquire::Text::new("Enter display name (optional):")
            .with_default(&username)
            .prompt()
            .context("Failed to read display name")?;
        if prompt == username {
            None
        } else {
            Some(prompt)
        }
    };
    
    // Prompt for description if not provided (only in interactive mode)
    let list_description = if let Some(d) = description {
        Some(d)
    } else if state.is_some() {
        // In REPL mode, prompt for description
        let prompt = inquire::Text::new("Enter RankSet description (optional):")
            .prompt()
            .context("Failed to read description")?;
        if prompt.trim().is_empty() {
            None
        } else {
            Some(prompt)
        }
    } else {
        // In direct mode, no description
        None
    };
    
    // Create the list
    let list_author = author.unwrap_or_else(|| username.clone());
    let mut list = RankSet::new(name.clone(), list_author, list_description);
    
    // Add the initial user
    let user = User::new(username.clone(), user_display_name);
    let user_id = user.id.to_string();
    list.add_user(user)?;
    
    // Set the file path
    let filename = format!("{}.rankset", name);
    list.file_path = Some(PathBuf::from(&filename));
    
    // Save the list
    list.save()?;
    
    println!("✓ Created: {}", filename);
    println!("✓ Active user: {} ({})", username, user_id);
    
    // If in REPL mode, load the list into state
    if let Some(state) = state {
        state.rankset = Some(list);
        state.active_user_id = Some(user_id.into());
        println!("✓ RankSet loaded into session");
    } else {
        println!("\nNext steps:");
        println!("  rankhaus load {}      # Load the list", filename);
        println!("  rankhaus items add         # Add items to rank");
    }
    
    Ok(())
}
