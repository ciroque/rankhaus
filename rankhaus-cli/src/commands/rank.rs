use crate::state::AppState;
use anyhow::{bail, Context, Result};
use rankhaus::session::SessionStatus;
use rankhaus::strategy::merge::MergeStrategy;
use rankhaus::strategy::RankStrategy;
use rankhaus::Ranking;

pub fn start(state: Option<&mut AppState>) -> Result<()> {
    let app_state = state.ok_or_else(|| anyhow::anyhow!("No state available"))?;

    // Check prerequisites
    let rankset = app_state
        .rankset
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded. Use 'init' or 'load' first."))?;

    let active_user_id = app_state
        .active_user_id
        .as_ref()
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

    // Create ranking object to track session
    let mut ranking = Ranking::new(active_user_id.clone(), app_state.active_strategy.clone());
    let session_id = ranking.session.info.id.clone();

    // Perform ranking
    perform_ranking(rankset, &mut strategy, &mut ranking, session_id)
}

pub fn resume(session_id: String, state: Option<&mut AppState>) -> Result<()> {
    let app_state = state.ok_or_else(|| anyhow::anyhow!("No state available"))?;

    // Check prerequisites
    let rankset = app_state
        .rankset
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("No rankset loaded. Use 'init' or 'load' first."))?;

    // Find the in-progress session
    let ranking_idx = rankset
        .rankings
        .iter()
        .position(|r| {
            let id_str = r.session.info.id.as_str();
            (id_str == session_id || id_str.starts_with(&session_id))
                && r.session.info.status == SessionStatus::InProgress
        })
        .ok_or_else(|| anyhow::anyhow!("No in-progress session found with ID '{}'", session_id))?;

    let mut ranking = rankset.rankings.remove(ranking_idx);
    let session_id = ranking.session.info.id.clone();

    println!("\n🔄 Resuming ranking session: {}", session_id.as_str());
    println!(
        "Comparisons completed: {}",
        ranking.session.comparisons.len()
    );
    println!();

    // Create strategy with all items
    let item_ids: Vec<_> = rankset.items.keys().map(|k| k.clone().into()).collect();
    let mut strategy = MergeStrategy::new(item_ids);

    // Replay all saved comparisons to rebuild strategy state
    println!("Restoring session state...");
    for comparison in &ranking.session.comparisons {
        let item_a = rankset.get_item(&comparison.a.to_string())?;
        let item_b = rankset.get_item(&comparison.b.to_string())?;
        strategy.compare(item_a, item_b, &comparison.winner)?;
    }
    println!(
        "✓ Restored {} comparisons\n",
        ranking.session.comparisons.len()
    );

    // Continue ranking
    perform_ranking(rankset, &mut strategy, &mut ranking, session_id)
}

fn perform_ranking(
    rankset: &mut rankhaus::RankSet,
    strategy: &mut MergeStrategy,
    ranking: &mut Ranking,
    session_id: rankhaus::Id,
) -> Result<()> {
    // Estimate total comparisons for merge sort (worst case: n * log2(n))
    let n = rankset.items.len() as f64;
    let estimated_total = (n * n.log2()).ceil() as usize;

    // Track comparisons made in this session (not including resumed ones)
    let initial_count = ranking.session.comparisons.len();

    // Perform comparisons
    while let Some((a_id, b_id)) = strategy.next_comparison() {
        let item_a = rankset.get_item(&a_id.to_string())?;
        let item_b = rankset.get_item(&b_id.to_string())?;

        let current_count = ranking.session.comparisons.len() + 1;

        // Display comparison
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Comparison {} of ~{}", current_count, estimated_total);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("  1️⃣  {}", item_a.value);
        println!();
        println!("  2️⃣  {}", item_b.value);
        println!();
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Get user choice
        let choice = loop {
            print!("Which is better? (1, 2, or 'q' to quit): ");
            use std::io::{self, Write};
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "1" => break Some(1),
                "2" => break Some(2),
                "q" | "Q" | "quit" => break None,
                _ => println!("Invalid choice. Please enter 1, 2, or 'q' to quit."),
            }
        };

        // Check if user wants to quit
        let choice = match choice {
            Some(c) => c,
            None => {
                // Save progress and exit
                rankset.rankings.retain(|r| r.session.info.id != session_id);
                rankset.rankings.push(ranking.clone());
                rankset.save().context("Failed to save progress")?;

                println!("\n⏸️  Session suspended");
                println!(
                    "✓ Progress saved ({} comparisons)",
                    ranking.session.comparisons.len()
                );
                println!("Resume with: sessions resume {}", session_id.as_str());
                return Ok(());
            }
        };

        let winner = if choice == 1 { item_a } else { item_b };
        strategy.compare(item_a, item_b, &winner.id)?;

        // Record comparison in session
        ranking
            .session
            .add_comparison(item_a.id.clone(), item_b.id.clone(), winner.id.clone());

        // Save progress after each comparison
        // Remove existing session if it exists, then add updated one
        rankset.rankings.retain(|r| r.session.info.id != session_id);
        rankset.rankings.push(ranking.clone());
        rankset.save().context("Failed to save progress")?;

        println!();
    }

    // Finalize ranking
    let result = strategy.finalize()?;
    let order = result
        .order
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No ranking order produced"))?;

    // Update ranking with final result and mark as complete
    ranking.result = Some(result.clone());
    ranking.session.info.complete();

    let total_comparisons = ranking.session.comparisons.len();

    // Clear comparisons now that ranking is complete (save space)
    ranking.session.comparisons.clear();

    // Save final ranking
    rankset.rankings.retain(|r| r.session.info.id != session_id);
    rankset.rankings.push(ranking.clone());
    rankset.save().context("Failed to save rankset")?;

    // Display results
    println!("\n✅ Ranking complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Final Ranking ({} comparisons)", total_comparisons);
    if initial_count > 0 {
        println!(
            "  ({} resumed + {} new)",
            initial_count,
            total_comparisons - initial_count
        );
    }
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
