use crate::commands::{init, load};
use crate::state::AppState;
use crate::RanksetsCommands;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn execute(command: RanksetsCommands, state: Option<&mut AppState>) -> Result<()> {
    match command {
        RanksetsCommands::List => list(state),
        RanksetsCommands::Load { file } => load::execute(file, state),
        RanksetsCommands::New {
            name,
            user,
            display_name,
            description,
            author,
        } => init::execute(name, user, display_name, description, author, state),
    }
}

fn list(state: Option<&mut AppState>) -> Result<()> {
    let ranksets_dir = Path::new("ranksets");

    if !ranksets_dir.exists() {
        println!("No ranksets directory found.");
        println!("Create one with: mkdir ranksets");
        return Ok(());
    }

    // Read all .rankset files
    let entries = fs::read_dir(ranksets_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "rankset")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    if entries.is_empty() {
        println!("No ranksets found in ./ranksets/");
        return Ok(());
    }

    // Get the currently loaded rankset path if any
    let current_path = state
        .and_then(|s| s.rankset.as_ref())
        .and_then(|r| r.file_path.as_ref());

    println!("\n{:-<80}", "");
    println!("{:<30} {:<10} Description", "Rankset", "Items");
    println!("{:-<80}", "");

    for entry in entries {
        let path = entry.path();
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        // Check if this is the currently loaded rankset
        let is_current = current_path.map(|cp| cp == &path).unwrap_or(false);
        let marker = if is_current { "*" } else { " " };

        // Try to load and parse the rankset to get metadata
        match fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(json) => {
                        let items_count =
                            json["items"].as_object().map(|obj| obj.len()).unwrap_or(0);

                        let description = json["meta"]["description"].as_str().unwrap_or("");

                        // Truncate description if too long
                        let desc_display = if description.len() > 35 {
                            format!("{}...", &description[..32])
                        } else {
                            description.to_string()
                        };

                        println!("{} {:<29} {:<10} {}", marker, filename, items_count, desc_display);
                    }
                    Err(_) => {
                        println!("{} {:<29} {:<10} (invalid JSON)", marker, filename, "?");
                    }
                }
            }
            Err(_) => {
                println!("{} {:<29} {:<10} (cannot read)", marker, filename, "?");
            }
        }
    }

    println!();
    println!("Load a rankset with: ranksets load <name>");
    println!();

    Ok(())
}
