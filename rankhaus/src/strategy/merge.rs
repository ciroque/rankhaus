use crate::{Id, Item, Result, strategy::{RankResult, RankStrategy}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Merge sort based ranking strategy
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeStrategy {
    items: Vec<Id>,
    comparisons: HashMap<(String, String), String>,
    state: MergeState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MergeState {
    /// Stack of merge operations to perform
    merge_stack: Vec<MergeOp>,
    /// Current sorted result being built
    sorted: Vec<Id>,
    /// Whether the sort is complete
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MergeOp {
    left: Vec<Id>,
    right: Vec<Id>,
    left_idx: usize,
    right_idx: usize,
    result: Vec<Id>,
    /// Index of the merge operation that produces the left input (if any)
    left_source: Option<usize>,
    /// Index of the merge operation that produces the right input (if any)
    right_source: Option<usize>,
}

impl MergeStrategy {
    pub fn new(items: Vec<Id>) -> Self {
        let mut strategy = Self {
            items: items.clone(),
            comparisons: HashMap::new(),
            state: MergeState {
                merge_stack: Vec::new(),
                sorted: Vec::new(),
                completed: false,
            },
        };
        
        // Initialize merge sort
        if items.is_empty() {
            strategy.state.completed = true;
        } else if items.len() == 1 {
            strategy.state.sorted = items.clone();
            strategy.state.completed = true;
        } else {
            strategy.initialize_merge_sort();
        }
        
        strategy
    }
    
    fn initialize_merge_sort(&mut self) {
        // Start with singleton lists - these are already "sorted"
        let mut sublists: Vec<Vec<Id>> = self.items.iter()
            .map(|id| vec![id.clone()])
            .collect();
        
        // Track which merge operation index produces each sublist
        let mut sublist_sources: Vec<Option<usize>> = vec![None; sublists.len()];
        
        // Build merge operations level by level, bottom-up
        while sublists.len() > 1 {
            let mut next_level = Vec::new();
            let mut next_sources = Vec::new();
            let mut i = 0;
            
            while i < sublists.len() {
                if i + 1 < sublists.len() {
                    // Pair exists, create merge operation for these TWO SORTED lists
                    let op_idx = self.state.merge_stack.len();
                    let op = MergeOp {
                        left: sublists[i].clone(),
                        right: sublists[i + 1].clone(),
                        left_idx: 0,
                        right_idx: 0,
                        result: Vec::new(),
                        left_source: sublist_sources[i],
                        right_source: sublist_sources[i + 1],
                    };
                    self.state.merge_stack.push(op);
                    
                    // The result will be filled by comparisons
                    let mut merged = Vec::new();
                    merged.extend(sublists[i].iter().cloned());
                    merged.extend(sublists[i + 1].iter().cloned());
                    next_level.push(merged);
                    next_sources.push(Some(op_idx));
                    i += 2;
                } else {
                    // Odd one out, carry forward as-is (already sorted)
                    next_level.push(sublists[i].clone());
                    next_sources.push(sublist_sources[i]);
                    i += 1;
                }
            }
            
            sublists = next_level;
            sublist_sources = next_sources;
        }
    }
    
    fn get_comparison_key(&self, a: &Id, b: &Id) -> (String, String) {
        let a_str = a.to_string();
        let b_str = b.to_string();
        if a_str < b_str {
            (a_str, b_str)
        } else {
            (b_str, a_str)
        }
    }
    
    fn get_winner<'a>(&self, a: &'a Id, b: &'a Id) -> Option<&'a Id> {
        let key = self.get_comparison_key(a, b);
        self.comparisons.get(&key).map(|winner| {
            if winner == &a.to_string() { a } else { b }
        })
    }
}

impl RankStrategy for MergeStrategy {
    fn name(&self) -> &'static str {
        "merge"
    }
    
    fn compare(&mut self, a: &Item, b: &Item, winner_id: &Id) -> Result<()> {
        // Store the comparison result
        let key = self.get_comparison_key(&a.id, &b.id);
        self.comparisons.insert(key, winner_id.to_string());
        
        // Process merge operations
        self.process_merges();
        
        Ok(())
    }
    
    fn finalize(&mut self) -> Result<RankResult> {
        if !self.state.completed {
            return Err(crate::Error::Other("Ranking not complete".to_string()));
        }
        
        Ok(RankResult {
            order: Some(self.state.sorted.clone()),
            ratings: None,
        })
    }
    
    fn serialize_state(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self.state)?)
    }
    
    fn deserialize_state(&mut self, state: serde_json::Value) -> Result<()> {
        self.state = serde_json::from_value(state)?;
        Ok(())
    }
    
    fn next_comparison(&self) -> Option<(Id, Id)> {
        // Find the next pair that needs comparison
        for op in &self.state.merge_stack {
            if op.left_idx < op.left.len() && op.right_idx < op.right.len() {
                let left_item = &op.left[op.left_idx];
                let right_item = &op.right[op.right_idx];
                
                // Check if we already have this comparison
                if self.get_winner(left_item, right_item).is_none() {
                    return Some((left_item.clone(), right_item.clone()));
                }
            }
        }
        
        None
    }
    
    fn is_complete(&self) -> bool {
        self.state.completed
    }
}

impl MergeStrategy {
    fn process_merges(&mut self) {
        let comparisons = &self.comparisons;
        let mut completed_ops = Vec::new();
        
        // First pass: update inputs from completed source operations
        for idx in 0..self.state.merge_stack.len() {
            let (left_source, right_source) = {
                let op = &self.state.merge_stack[idx];
                (op.left_source, op.right_source)
            };
            
            // Update left input if it comes from a completed merge AND hasn't been updated yet
            if let Some(source_idx) = left_source {
                if self.state.merge_stack[source_idx].left_idx == self.state.merge_stack[source_idx].left.len() &&
                   self.state.merge_stack[source_idx].right_idx == self.state.merge_stack[source_idx].right.len() &&
                   !self.state.merge_stack[source_idx].result.is_empty() {
                    let result = self.state.merge_stack[source_idx].result.clone();
                    // Only update if we haven't started this merge yet
                    if self.state.merge_stack[idx].left_idx == 0 && self.state.merge_stack[idx].result.is_empty() {
                        self.state.merge_stack[idx].left = result;
                        self.state.merge_stack[idx].left_source = None; // Mark as updated
                    }
                }
            }
            
            // Update right input if it comes from a completed merge AND hasn't been updated yet
            if let Some(source_idx) = right_source {
                if self.state.merge_stack[source_idx].left_idx == self.state.merge_stack[source_idx].left.len() &&
                   self.state.merge_stack[source_idx].right_idx == self.state.merge_stack[source_idx].right.len() &&
                   !self.state.merge_stack[source_idx].result.is_empty() {
                    let result = self.state.merge_stack[source_idx].result.clone();
                    // Only update if we haven't started this merge yet
                    if self.state.merge_stack[idx].right_idx == 0 && self.state.merge_stack[idx].result.is_empty() {
                        self.state.merge_stack[idx].right = result;
                        self.state.merge_stack[idx].right_source = None; // Mark as updated
                    }
                }
            }
        }
        
        // Second pass: process merges (only if their inputs are ready)
        for (idx, op) in self.state.merge_stack.iter_mut().enumerate() {
            // Skip if this operation depends on incomplete sources
            if op.left_source.is_some() || op.right_source.is_some() {
                continue;
            }
            
            let mut made_progress = true;
            
            while made_progress && op.left_idx < op.left.len() && op.right_idx < op.right.len() {
                let left_item = &op.left[op.left_idx];
                let right_item = &op.right[op.right_idx];
                
                // Check winner using local comparisons reference
                let key = Self::make_comparison_key(left_item, right_item);
                if let Some(winner_str) = comparisons.get(&key) {
                    let winner = if winner_str == &left_item.to_string() {
                        left_item
                    } else {
                        right_item
                    };
                    
                    op.result.push(winner.clone());
                    if winner == left_item {
                        op.left_idx += 1;
                    } else {
                        op.right_idx += 1;
                    }
                } else {
                    made_progress = false;
                }
            }
            
            // Append remaining *only* if one side is exhausted
            if op.left_idx == op.left.len() {
                while op.right_idx < op.right.len() {
                    op.result.push(op.right[op.right_idx].clone());
                    op.right_idx += 1;
                }
            } else if op.right_idx == op.right.len() {
                while op.left_idx < op.left.len() {
                    op.result.push(op.left[op.left_idx].clone());
                    op.left_idx += 1;
                }
                // No else: if both have remainders, stay stuck until comparison arrives
            }
            
            // Check if this operation is complete
            if op.left_idx == op.left.len() && op.right_idx == op.right.len() {
                completed_ops.push(idx);
            }
        }
        
        // Check if the top-level merge (LAST operation in stack) is complete
        let last_idx = self.state.merge_stack.len().saturating_sub(1);
        if !self.state.merge_stack.is_empty() && completed_ops.contains(&last_idx) {
            // The final sorted result is in the LAST operation
            // which represents the top-level merge
            if let Some(last_op) = self.state.merge_stack.last() {
                self.state.sorted = last_op.result.clone();
                self.state.completed = true;
            }
        }
    }
    
    fn make_comparison_key(a: &Id, b: &Id) -> (String, String) {
        let a_str = a.to_string();
        let b_str = b.to_string();
        if a_str < b_str {
            (a_str, b_str)
        } else {
            (b_str, a_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Item;
    
    fn create_test_items(count: usize) -> Vec<Item> {
        (0..count)
            .map(|i| Item::new(format!("item{}", i)))
            .collect()
    }
    
    #[test]
    fn test_merge_strategy_creation() {
        let items = create_test_items(3);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let strategy = MergeStrategy::new(ids.clone());
        assert_eq!(strategy.name(), "merge");
        assert!(!strategy.is_complete());
        assert_eq!(strategy.items, ids);
    }
    
    #[test]
    fn test_empty_list() {
        let strategy = MergeStrategy::new(vec![]);
        assert!(strategy.is_complete());
        assert!(strategy.next_comparison().is_none());
    }
    
    #[test]
    fn test_single_item() {
        let items = create_test_items(1);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let strategy = MergeStrategy::new(ids);
        assert!(strategy.is_complete());
        assert!(strategy.next_comparison().is_none());
    }
    
    #[test]
    fn test_two_items() {
        let items = create_test_items(2);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let mut strategy = MergeStrategy::new(ids.clone());
        assert!(!strategy.is_complete());
        
        // Should need one comparison
        let (a, b) = strategy.next_comparison().unwrap();
        assert!(a == ids[0] || a == ids[1]);
        assert!(b == ids[0] || b == ids[1]);
        assert_ne!(a, b);
        
        // Make the comparison
        strategy.compare(&items[0], &items[1], &items[0].id).unwrap();
        
        assert!(strategy.is_complete());
        let result = strategy.finalize().unwrap();
        assert_eq!(result.order.unwrap(), vec![items[0].id.clone(), items[1].id.clone()]);
    }
    
    #[test]
    fn test_three_items() {
        let items = create_test_items(3);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let mut strategy = MergeStrategy::new(ids.clone());
        assert!(!strategy.is_complete());
        
        // Perform comparisons until complete
        let mut comparison_count = 0;
        while let Some((a_id, b_id)) = strategy.next_comparison() {
            let a = items.iter().find(|item| item.id == a_id).unwrap();
            let b = items.iter().find(|item| item.id == b_id).unwrap();
            
            // Always prefer item0 > item1 > item2
            let winner = if a.value < b.value { a } else { b };
            strategy.compare(a, b, &winner.id).unwrap();
            
            comparison_count += 1;
            if comparison_count > 10 {
                panic!("Too many comparisons");
            }
        }
        
        assert!(strategy.is_complete());
        assert!(comparison_count <= 3); // Merge sort should need at most 3 comparisons for 3 items
        
        let result = strategy.finalize().unwrap();
        let order = result.order.unwrap();
        assert_eq!(order.len(), 3);
        
        // Verify order is correct (item0 < item1 < item2)
        assert_eq!(order[0], items[0].id);
        assert_eq!(order[1], items[1].id);
        assert_eq!(order[2], items[2].id);
    }
    
    #[test]
    fn test_four_items() {
        let items = create_test_items(4);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let mut strategy = MergeStrategy::new(ids.clone());
        
        let mut comparison_count = 0;
        while let Some((a_id, b_id)) = strategy.next_comparison() {
            let a = items.iter().find(|item| item.id == a_id).unwrap();
            let b = items.iter().find(|item| item.id == b_id).unwrap();
            
            let winner = if a.value < b.value { a } else { b };
            strategy.compare(a, b, &winner.id).unwrap();
            
            comparison_count += 1;
            if comparison_count > 20 {
                panic!("Too many comparisons");
            }
        }
        
        assert!(strategy.is_complete());
        assert!(comparison_count <= 5); // Merge sort needs at most 5 comparisons for 4 items
        
        let result = strategy.finalize().unwrap();
        let order = result.order.unwrap();
        assert_eq!(order.len(), 4);
        
        // Verify order is correct (item0 < item1 < item2 < item3)
        assert_eq!(order[0], items[0].id);
        assert_eq!(order[1], items[1].id);
        assert_eq!(order[2], items[2].id);
        assert_eq!(order[3], items[3].id);
    }
    
    #[test]
    fn test_finalize_before_complete() {
        let items = create_test_items(2);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let mut strategy = MergeStrategy::new(ids);
        let result = strategy.finalize();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_six_items() {
        let items = create_test_items(6);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let mut strategy = MergeStrategy::new(ids.clone());
        
        // Debug: print merge stack structure
        println!("Merge stack has {} operations:", strategy.state.merge_stack.len());
        for (i, op) in strategy.state.merge_stack.iter().enumerate() {
            println!("  Op {}: left={:?}, right={:?}", i, 
                op.left.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                op.right.iter().map(|id| id.to_string()).collect::<Vec<_>>());
        }
        
        let mut comparison_count = 0;
        while let Some((a_id, b_id)) = strategy.next_comparison() {
            let a = items.iter().find(|item| item.id == a_id).unwrap();
            let b = items.iter().find(|item| item.id == b_id).unwrap();
            
            let winner = if a.value < b.value { a } else { b };
            strategy.compare(a, b, &winner.id).unwrap();
            
            comparison_count += 1;
            println!("Comparison {}: {} vs {} -> {}", comparison_count, a.value, b.value, winner.value);
            println!("  Complete: {}", strategy.is_complete());
            
            // Debug: show which operations are ready
            let ready_ops: Vec<usize> = strategy.state.merge_stack.iter().enumerate()
                .filter(|(_, op)| op.left_source.is_none() && op.right_source.is_none())
                .map(|(i, _)| i)
                .collect();
            println!("  Ready ops: {:?}", ready_ops);
            
            if comparison_count > 30 {
                panic!("Too many comparisons");
            }
        }
        
        println!("Total comparisons: {}", comparison_count);
        assert!(strategy.is_complete());
        assert!(comparison_count >= 8); // Should need at least 8 comparisons for 6 items
        assert!(comparison_count <= 15); // But not more than 15
        
        let result = strategy.finalize().unwrap();
        let order = result.order.unwrap();
        assert_eq!(order.len(), 6);
        
        // Verify order is correct
        for i in 0..6 {
            assert_eq!(order[i], items[i].id);
        }
    }
    
    #[test]
    fn test_serialize_deserialize() {
        let items = create_test_items(3);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        
        let mut strategy = MergeStrategy::new(ids.clone());
        
        // Make one comparison
        if let Some((a_id, b_id)) = strategy.next_comparison() {
            let a = items.iter().find(|item| item.id == a_id).unwrap();
            let b = items.iter().find(|item| item.id == b_id).unwrap();
            strategy.compare(a, b, &a.id).unwrap();
        }
        
        // Serialize state
        let state = strategy.serialize_state().unwrap();
        
        // Create new strategy and deserialize
        let mut new_strategy = MergeStrategy::new(ids);
        new_strategy.deserialize_state(state).unwrap();
        
        // States should match
        assert_eq!(strategy.is_complete(), new_strategy.is_complete());
    }
}
