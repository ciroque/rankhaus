use anyhow::{bail, Context, Result};
use crate::state::AppState;
use crate::ItemsCommands;
use rankhaus::Item;
use std::io::{self, BufRead};

pub fn execute(command: ItemsCommands, state: Option<&mut AppState>) -> Result<()> {
    // Check if list is loaded
    let has_rankset = state.as_ref().map(|s| s.has_rankset()).unwrap_or(false);
    if !has_rankset {
        bail!("No list loaded. Use 'init <name>' or 'load <file>' first.");
    }
    
    match command {
        ItemsCommands::List => {
            list(state)
        }
        ItemsCommands::Add { item } => {
            add(state, item)
        }
        ItemsCommands::Remove => {
            remove(state)
        }
        ItemsCommands::Edit { identifier, new_value } => {
            edit(state, identifier, new_value)
        }
    }
}

fn list(state: Option<&mut AppState>) -> Result<()> {
    let list = state
        .and_then(|s| s.rankset.as_ref())
        .ok_or_else(|| anyhow::anyhow!("No list loaded"))?;
    
    if list.items.is_empty() {
        println!("No items yet. Use 'items add' to add some.");
        return Ok(());
    }
    
    // Collect and sort items by ID for consistent display
    let mut items: Vec<_> = list.items.values().collect();
    items.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    
    println!("\nItems ({})", items.len());
    println!("{:-<60}", "");
    println!("{:<10} {}", "ID", "Value");
    println!("{:-<60}", "");
    
    for item in items {
        println!("{:<10} {}", item.id.as_str(), item.value);
    }
    
    println!();
    Ok(())
}

fn add(state: Option<&mut AppState>, item_arg: Option<String>) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;
    
    let mut added = 0;
    let mut skipped = 0;
    
    // If item provided as argument, add it directly
    if let Some(value) = item_arg {
        let value = value.trim();
        
        if value.is_empty() {
            bail!("Item value cannot be empty");
        }
        
        // Check for duplicates
        if rankset.items.values().any(|item| item.value == value) {
            bail!("Item '{}' already exists", value);
        }
        
        // Add the item
        let item = Item::new(value.to_string());
        rankset.add_item(item)?;
        
        // Auto-save
        rankset.save().context("Failed to save rankset")?;
        
        println!("✓ Added: {}", value);
        return Ok(());
    }
    
    // Interactive mode
    println!("Enter items (one per line, empty line to finish):");
    print!("> ");
    use std::io::Write;
    io::stdout().flush()?;
    
    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line")?;
        let value = line.trim();
        
        // Empty line means done
        if value.is_empty() {
            break;
        }
        
        // Check for duplicates
        if rankset.items.values().any(|item| item.value == value) {
            eprintln!("⚠ Skipped duplicate: {}", value);
            skipped += 1;
            print!("> ");
            io::stdout().flush()?;
            continue;
        }
        
        // Add the item
        let item = Item::new(value.to_string());
        rankset.add_item(item)?;
        println!("  ✓ {}", value);
        added += 1;
        
        // Prompt for next line
        print!("> ");
        io::stdout().flush()?;
    }
    
    // Auto-save
    rankset.save().context("Failed to save rankset")?;
    
    if added > 0 || skipped > 0 {
        println!();
        if added > 0 {
            println!("✓ Added {} item(s)", added);
        }
        if skipped > 0 {
            println!("⚠ Skipped {} duplicate(s)", skipped);
        }
    }
    
    Ok(())
}

fn remove(state: Option<&mut AppState>) -> Result<()> {
    let list = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No list loaded"))?;
    
    if list.items.is_empty() {
        println!("No items to remove.");
        return Ok(());
    }
    
    // Show current items
    println!("\nCurrent items:");
    let mut items: Vec<_> = list.items.values().collect();
    items.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    for item in &items {
        println!("  {} - {}", item.id.as_str(), item.value);
    }
    println!();
    
    // Prompt for items to remove
    println!("Enter item IDs or values to remove, one per line.");
    println!("Press Ctrl+D (or Ctrl+Z on Windows) when done:");
    println!();
    
    let stdin = io::stdin();
    let mut removed = 0;
    let mut not_found = 0;
    
    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line")?;
        let identifier = line.trim();
        
        if identifier.is_empty() {
            continue;
        }
        
        match list.remove_item(identifier) {
            Ok(_) => {
                println!("✓ Removed: {}", identifier);
                removed += 1;
            }
            Err(_) => {
                eprintln!("⚠ Not found: {}", identifier);
                not_found += 1;
            }
        }
    }
    
    // Auto-save if anything was removed
    if removed > 0 {
        list.save().context("Failed to save list")?;
    }
    
    println!();
    println!("✓ Removed {} item(s)", removed);
    if not_found > 0 {
        println!("⚠ {} not found", not_found);
    }
    
    Ok(())
}

fn edit(state: Option<&mut AppState>, identifier: String, new_value: String) -> Result<()> {
    let list = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No list loaded"))?;
    
    // Check if new value already exists (and it's not the same item)
    if let Some(existing) = list.items.values().find(|item| item.value == new_value) {
        if existing.id.as_str() != identifier && existing.value != identifier {
            bail!("Item with value '{}' already exists", new_value);
        }
    }
    
    // Get the item to edit
    let item = list.get_item_mut(&identifier)
        .context(format!("Item not found: {}", identifier))?;
    
    let old_value = item.value.clone();
    item.value = new_value.clone();
    
    // Auto-save
    list.save().context("Failed to save list")?;
    
    println!("✓ Updated: '{}' → '{}'", old_value, new_value);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rankhaus::{RankSet, User, Item};
    use std::path::PathBuf;
    
    fn create_test_state() -> AppState {
        let mut list = RankSet::new(
            "test".to_string(),
            "author".to_string(),
            "description".to_string(),
        );
        list.file_path = Some(PathBuf::from("test_items.rankhaus.json"));
        
        let user = User::new("testuser".to_string(), None);
        list.add_user(user).unwrap();
        
        AppState {
            rankset: Some(list),
            active_user_id: None,
            active_strategy: "merge".to_string(),
        }
    }
    
    #[test]
    fn test_list_empty() {
        let mut state = create_test_state();
        let result = list(Some(&mut state));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_list_with_items() {
        let mut state = create_test_state();
        state.rankset.as_mut().unwrap().add_item(Item::new("red".to_string())).unwrap();
        state.rankset.as_mut().unwrap().add_item(Item::new("blue".to_string())).unwrap();
        
        let result = list(Some(&mut state));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_list_no_state() {
        let result = list(None);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_edit_by_id() {
        let mut state = create_test_state();
        let item = Item::new("red".to_string());
        let item_id = item.id.to_string();
        state.rankset.as_mut().unwrap().add_item(item).unwrap();
        
        let result = edit(Some(&mut state), item_id.clone(), "crimson".to_string());
        assert!(result.is_ok());
        
        let updated = state.rankset.as_ref().unwrap().get_item(&item_id).unwrap();
        assert_eq!(updated.value, "crimson");
    }
    
    #[test]
    fn test_edit_by_value() {
        let mut state = create_test_state();
        state.rankset.as_mut().unwrap().add_item(Item::new("red".to_string())).unwrap();
        
        let result = edit(Some(&mut state), "red".to_string(), "crimson".to_string());
        assert!(result.is_ok());
        
        let updated = state.rankset.as_ref().unwrap().get_item("crimson").unwrap();
        assert_eq!(updated.value, "crimson");
    }
    
    #[test]
    fn test_edit_not_found() {
        let mut state = create_test_state();
        
        let result = edit(Some(&mut state), "nonexistent".to_string(), "new".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
    
    #[test]
    fn test_edit_duplicate_value() {
        let mut state = create_test_state();
        state.rankset.as_mut().unwrap().add_item(Item::new("red".to_string())).unwrap();
        state.rankset.as_mut().unwrap().add_item(Item::new("blue".to_string())).unwrap();
        
        let result = edit(Some(&mut state), "red".to_string(), "blue".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
    
    #[test]
    fn test_edit_no_state() {
        let result = edit(None, "id".to_string(), "value".to_string());
        assert!(result.is_err());
    }
}
