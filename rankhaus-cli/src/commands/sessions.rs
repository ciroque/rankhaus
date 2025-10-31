use anyhow::{bail, Context, Result};
use crate::state::AppState;
use crate::SessionsCommands;
use rankhaus::session::SessionStatus;

pub fn execute(command: SessionsCommands, state: Option<&mut AppState>) -> Result<()> {
    // Check if list is loaded
    let has_rankset = state.as_ref().map(|s| s.has_rankset()).unwrap_or(false);
    if !has_rankset {
        bail!("No rankset loaded. Use 'init <name>' or 'load <file>' first.");
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

fn list(state: Option<&mut AppState>) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_ref())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;
    
    if rankset.rankings.is_empty() {
        println!("No ranking sessions found.");
        return Ok(());
    }
    
    println!("\n{:-<60}", "");
    println!("{:<10} {:<12} {:<10} {:<10} {:<15}", "Session", "User", "Strategy", "Status", "Created");
    println!("{:-<60}", "");
    
    for ranking in &rankset.rankings {
        let user = rankset.users.get(&ranking.user_id.to_string())
            .map(|u| u.username.as_str())
            .unwrap_or("unknown");
        
        let status = match ranking.session.info.status {
            SessionStatus::InProgress => "in_progress",
            SessionStatus::Completed => "completed",
            SessionStatus::Abandoned => "abandoned",
        };
        
        let created = ranking.session.info.created.format("%Y-%m-%d %H:%M").to_string();
        
        println!("{:<10} {:<12} {:<10} {:<10} {:<15}", 
            ranking.session.info.id.as_str(),
            user,
            ranking.strategy,
            status,
            created
        );
    }
    
    println!();
    Ok(())
}

fn show(state: Option<&mut AppState>, session_id: String) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_ref())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;
    
    // Find the ranking by session ID (exact match or prefix)
    let ranking = rankset.rankings.iter()
        .find(|r| {
            let id_str = r.session.info.id.as_str();
            id_str == session_id || id_str.starts_with(&session_id)
        })
        .ok_or_else(|| anyhow::anyhow!("Session '{}' not found", session_id))?;
    
    // Get user info
    let user = rankset.users.get(&ranking.user_id.to_string())
        .map(|u| u.username.as_str())
        .unwrap_or("unknown");
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Session Details");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Session ID:   {}", ranking.session.info.id.as_str());
    println!("User:         {}", user);
    println!("Strategy:     {}", ranking.strategy);
    println!("Status:       {:?}", ranking.session.info.status);
    println!("Created:      {}", ranking.session.info.created.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Last Updated: {}", ranking.session.info.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));
    
    if let Some(completed) = ranking.session.info.completed {
        println!("Completed:    {}", completed.format("%Y-%m-%d %H:%M:%S UTC"));
    }
    
    println!("Comparisons:  {}", ranking.session.comparisons.len());
    println!();
    
    // Show ranking result if available
    if let Some(ref result) = ranking.result {
        if let Some(ref order) = result.order {
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("  Final Ranking");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();
            
            for (rank, item_id) in order.iter().enumerate() {
                if let Ok(item) = rankset.get_item(&item_id.to_string()) {
                    println!("  {}. {}", rank + 1, item.value);
                }
            }
            println!();
        }
    }
    
    Ok(())
}

fn delete(state: Option<&mut AppState>, session_id: String) -> Result<()> {
    let rankset = state
        .and_then(|s| s.rankset.as_mut())
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded"))?;
    
    // Find the index of the ranking by session ID (exact match or prefix)
    let index = rankset.rankings.iter()
        .position(|r| {
            let id_str = r.session.info.id.as_str();
            id_str == session_id || id_str.starts_with(&session_id)
        })
        .ok_or_else(|| anyhow::anyhow!("Session '{}' not found", session_id))?;
    
    let removed = rankset.rankings.remove(index);
    
    // Auto-save
    rankset.save().context("Failed to save rankset")?;
    
    println!("✓ Deleted session: {}", removed.session.info.id.as_str());
    
    Ok(())
}
