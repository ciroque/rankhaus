use anyhow::{bail, Context, Result};
use crate::state::AppState;
use rankhaus::Ranking;
use rankhaus::strategy::merge::MergeStrategy;
use rankhaus::strategy::RankStrategy;

pub fn execute(state: Option<&mut AppState>) -> Result<()> {
    let app_state = state.ok_or_else(|| anyhow::anyhow!("No state available"))?;
    
    // Check prerequisites
    let rankset = app_state.rankset.as_mut()
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded. Use 'init' or 'load' first."))?;
    
    let active_user_id = app_state.active_user_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active user. Use 'users select <user>' first."))?;
    
    // Check if we have items to rank
    if rankset.items.is_empty() {
        bail!("No items to rank. Use 'items add' to add items first.");
    }
    
    if rankset.items.len() == 1 {
        bail!("Need at least 2 items to rank.");
    }
    
    // Get user info
    let user = rankset.get_user(&active_user_id.to_string())?;
    println!("\n🎯 Starting ranking session for user: {}", user.username);
    println!("Items to rank: {}", rankset.items.len());
    println!();
    
    // Create strategy
    let item_ids: Vec<_> = rankset.items.keys().map(|k| k.clone().into()).collect();
    let mut strategy = MergeStrategy::new(item_ids);
    
    // Perform comparisons
    let mut comparison_count = 0;
    while let Some((a_id, b_id)) = strategy.next_comparison() {
        let item_a = rankset.get_item(&a_id.to_string())?;
        let item_b = rankset.get_item(&b_id.to_string())?;
        
        // Display comparison
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Comparison #{}", comparison_count + 1);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("  1️⃣  {}", item_a.value);
        println!();
        println!("  2️⃣  {}", item_b.value);
        println!();
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Get user choice
        let choice = loop {
            print!("Which is better? (1 or 2): ");
            use std::io::{self, Write};
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => break 1,
                "2" => break 2,
                _ => println!("Invalid choice. Please enter 1 or 2."),
            }
        };
        
        let winner = if choice == 1 { item_a } else { item_b };
        strategy.compare(item_a, item_b, &winner.id)?;
        
        comparison_count += 1;
        println!();
    }
    
    // Finalize ranking
    let result = strategy.finalize()?;
    let order = result.order.as_ref().ok_or_else(|| anyhow::anyhow!("No ranking order produced"))?;
    
    // Create ranking object
    let mut ranking = Ranking::new(
        active_user_id.clone(),
        app_state.active_strategy.clone(),
    );
    ranking.result = Some(result.clone());
    
    // Save ranking
    rankset.rankings.push(ranking);
    rankset.save().context("Failed to save rankset")?;
    
    // Display results
    println!("\n✅ Ranking complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Final Ranking ({} comparisons)", comparison_count);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    for (rank, item_id) in order.iter().enumerate() {
        let item = rankset.get_item(&item_id.to_string())?;
        println!("  {}. {}", rank + 1, item.value);
    }
    
    println!();
    println!("✓ Ranking saved");
    
    Ok(())
}
