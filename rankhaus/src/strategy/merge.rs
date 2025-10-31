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
        // Start with singleton lists
        let mut sublists: Vec<Vec<Id>> = self.items.iter()
            .map(|id| vec![id.clone()])
            .collect();
        
        // Build merge operations bottom-up
        while sublists.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;
            
            while i < sublists.len() {
                if i + 1 < sublists.len() {
                    // Pair exists, create merge operation
                    let op = MergeOp {
                        left: sublists[i].clone(),
                        right: sublists[i + 1].clone(),
                        left_idx: 0,
                        right_idx: 0,
                        result: Vec::new(),
                    };
                    self.state.merge_stack.push(op);
                    
                    // Placeholder for merged result (will be filled by comparisons)
                    let mut merged = Vec::new();
                    merged.extend(sublists[i].iter().cloned());
                    merged.extend(sublists[i + 1].iter().cloned());
                    next_level.push(merged);
                    i += 2;
                } else {
                    // Odd one out, carry forward as-is
                    next_level.push(sublists[i].clone());
                    i += 1;
                }
            }
            
            sublists = next_level;
        }
        
        // The final result should be in sublists[0] once all merges complete
        // Reverse so we process merges in order
        self.state.merge_stack.reverse();
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
        
        for (idx, op) in self.state.merge_stack.iter_mut().enumerate() {
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
            
            // Add remaining items
            while op.left_idx < op.left.len() {
                op.result.push(op.left[op.left_idx].clone());
                op.left_idx += 1;
            }
            while op.right_idx < op.right.len() {
                op.result.push(op.right[op.right_idx].clone());
                op.right_idx += 1;
            }
            
            // Check if this operation is complete
            if op.left_idx == op.left.len() && op.right_idx == op.right.len() {
                completed_ops.push(idx);
            }
        }
        
        // Check if all operations are complete
        if !self.state.merge_stack.is_empty() && 
           completed_ops.len() == self.state.merge_stack.len() {
            // The final sorted result is in the FIRST operation (after reversing)
            // which represents the top-level merge
            if let Some(first_op) = self.state.merge_stack.first() {
                self.state.sorted = first_op.result.clone();
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
