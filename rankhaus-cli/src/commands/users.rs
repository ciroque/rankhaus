use crate::state::AppState;
use crate::UsersCommands;
use anyhow::{bail, Context, Result};
use rankhaus::User;

pub fn execute(command: UsersCommands, state: Option<&mut AppState>) -> Result<()> {
    // Check if list is loaded
    let has_rankset = state.as_ref().map(|s| s.has_rankset()).unwrap_or(false);
    if !has_rankset {
        bail!("No list loaded. Use 'init <name>' or 'load <file>' first.");
    }

    match command {
        UsersCommands::List => list(state),
        UsersCommands::Add {
            username,
            display_name,
        } => add(state, username, display_name),
        UsersCommands::Remove {
            identifier,
            cascade,
        } => remove(state, identifier, cascade),
        UsersCommands::Edit {
            identifier,
            new_display_name,
        } => edit(state, identifier, new_display_name),
        UsersCommands::Select { identifier } => select(state, identifier),
    }
}

fn list(state: Option<&mut AppState>) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_ref())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;

    if rankset.users.is_empty() {
        println!("No users yet. Use 'users add' to add some.");
        return Ok(());
    }

    // Collect and sort users by username
    let mut users: Vec<_> = rankset.users.values().collect();
    users.sort_by(|a, b| a.username.cmp(&b.username));

    println!("\nUsers ({})", users.len());
    println!("{:-<80}", "");
    println!("{:<10} {:<20} {:<30}", "ID", "Username", "Display Name");
    println!("{:-<80}", "");

    for user in users {
        println!(
            "{:<10} {:<20} {:<30}",
            user.id.as_str(),
            user.username,
            user.display_name
        );
    }

    println!();
    Ok(())
}

fn add(state: Option<&mut AppState>, username: String, display_name: Option<String>) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;

    // Check if username already exists
    if rankset.users.values().any(|u| u.username == username) {
        bail!("User '{}' already exists", username);
    }

    let user = User::new(username.clone(), display_name.clone());
    let user_id = user.id.to_string();
    rankset.add_user(user)?;

    // Auto-save
    rankset.save().context("Failed to save rankset")?;

    let display = display_name.as_ref().unwrap_or(&username);
    println!("✓ Added user: {} ({}) - {}", username, user_id, display);

    Ok(())
}

fn remove(state: Option<&mut AppState>, identifier: String, cascade: bool) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;

    // Get user info before removing
    let user = rankset.get_user(&identifier)?;
    let username = user.username.clone();
    let user_id = user.id.to_string();

    // Check for rankings
    let has_rankings = rankset
        .rankings
        .iter()
        .any(|r| r.user_id.as_str() == user_id);

    if has_rankings && !cascade {
        bail!(
            "User '{}' has rankings. Use --cascade to delete user and their rankings.",
            username
        );
    }

    // Remove user
    rankset.remove_user(&identifier, cascade)?;

    // Auto-save
    rankset.save().context("Failed to save rankset")?;

    if cascade && has_rankings {
        println!("✓ Removed user '{}' and their rankings", username);
    } else {
        println!("✓ Removed user '{}'", username);
    }

    Ok(())
}

fn edit(state: Option<&mut AppState>, identifier: String, new_display_name: String) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;

    let user = rankset
        .get_user_mut(&identifier)
        .context(format!("User not found: {}", identifier))?;

    let old_display = user.display_name.clone();
    user.display_name = new_display_name.clone();
    user.touch();

    // Auto-save
    rankset.save().context("Failed to save rankset")?;

    println!(
        "✓ Updated display name: '{}' → '{}'",
        old_display, new_display_name
    );

    Ok(())
}

fn select(state: Option<&mut AppState>, identifier: String) -> Result<()> {
    let app_state = state.ok_or_else(|| anyhow::anyhow!("No state available"))?;

    let rankset = app_state
        .rankset
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;

    // Verify user exists
    let user = rankset.get_user(&identifier)?;
    let user_id = user.id.clone();
    let username = user.username.clone();

    // Set active user
    app_state.active_user_id = Some(user_id);

    println!("✓ Active user: {} ({})", username, user.id.as_str());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rankhaus::{RankSet, User};
    use std::path::PathBuf;

    fn create_test_state() -> AppState {
        let mut rankset = RankSet::new("test".to_string(), "author".to_string(), None);
        rankset.file_path = Some(PathBuf::from("test_users.rankset"));

        let user = User::new("alice".to_string(), Some("Alice".to_string()));
        rankset.add_user(user).unwrap();

        AppState {
            rankset: Some(rankset),
            active_user_id: None,
            active_strategy: "merge".to_string(),
        }
    }

    #[test]
    fn test_list_empty() {
        let mut state = AppState::new();
        state.rankset = Some(RankSet::new("test".to_string(), "author".to_string(), None));
        state.rankset.as_mut().unwrap().file_path = Some(PathBuf::from("test.json"));

        let result = list(Some(&mut state));
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_with_users() {
        let mut state = create_test_state();
        let result = list(Some(&mut state));
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_no_state() {
        let result = list(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_user() {
        let mut state = create_test_state();
        let result = add(Some(&mut state), "bob".to_string(), Some("Bob".to_string()));
        assert!(result.is_ok());

        let rankset = state.rankset.as_ref().unwrap();
        assert_eq!(rankset.users.len(), 2);
        assert!(rankset.get_user("bob").is_ok());
    }

    #[test]
    fn test_add_duplicate_user() {
        let mut state = create_test_state();
        let result = add(Some(&mut state), "alice".to_string(), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_add_no_state() {
        let result = add(None, "bob".to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_edit_user() {
        let mut state = create_test_state();
        let result = edit(
            Some(&mut state),
            "alice".to_string(),
            "Alice Smith".to_string(),
        );
        assert!(result.is_ok());

        let rankset = state.rankset.as_ref().unwrap();
        let user = rankset.get_user("alice").unwrap();
        assert_eq!(user.display_name, "Alice Smith");
    }

    #[test]
    fn test_edit_user_by_id() {
        let mut state = create_test_state();
        let user_id = state
            .rankset
            .as_ref()
            .unwrap()
            .users
            .values()
            .next()
            .unwrap()
            .id
            .to_string();

        let result = edit(Some(&mut state), user_id.clone(), "New Name".to_string());
        assert!(result.is_ok());

        let rankset = state.rankset.as_ref().unwrap();
        let user = rankset.get_user(&user_id).unwrap();
        assert_eq!(user.display_name, "New Name");
    }

    #[test]
    fn test_edit_user_not_found() {
        let mut state = create_test_state();
        let result = edit(
            Some(&mut state),
            "nonexistent".to_string(),
            "Name".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_select_user() {
        let mut state = create_test_state();
        let result = select(Some(&mut state), "alice".to_string());
        assert!(result.is_ok());
        assert!(state.active_user_id.is_some());
    }

    #[test]
    fn test_select_user_by_id() {
        let mut state = create_test_state();
        let user_id = state
            .rankset
            .as_ref()
            .unwrap()
            .users
            .values()
            .next()
            .unwrap()
            .id
            .to_string();

        let result = select(Some(&mut state), user_id.clone());
        assert!(result.is_ok());
        assert_eq!(state.active_user_id.as_ref().unwrap().as_str(), user_id);
    }

    #[test]
    fn test_select_user_not_found() {
        let mut state = create_test_state();
        let result = select(Some(&mut state), "nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_user() {
        let mut state = create_test_state();

        // Add another user first
        add(Some(&mut state), "bob".to_string(), None).unwrap();
        assert_eq!(state.rankset.as_ref().unwrap().users.len(), 2);

        let result = remove(Some(&mut state), "bob".to_string(), false);
        assert!(result.is_ok());
        assert_eq!(state.rankset.as_ref().unwrap().users.len(), 1);
    }

    #[test]
    fn test_remove_user_not_found() {
        let mut state = create_test_state();
        let result = remove(Some(&mut state), "nonexistent".to_string(), false);
        assert!(result.is_err());
    }
}
