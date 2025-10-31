use crate::{
    strategy::{RankResult, RankStrategy},
    Id, Item, Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// QuickSort based ranking strategy
#[derive(Debug, Serialize, Deserialize)]
pub struct QuickSortStrategy {
    items: Vec<Id>,
    comparisons: HashMap<(String, String), String>,
    state: QuickSortState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuickSortState {
    /// Stack of partition operations to perform
    partition_stack: Vec<PartitionOp>,
    /// Final sorted result
    sorted: Vec<Id>,
    /// Whether the sort is complete
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PartitionOp {
    /// Items to partition
    items: Vec<Id>,
    /// Index of the pivot element
    pivot_idx: usize,
    /// Items less than pivot (being built)
    less: Vec<Id>,
    /// Items greater than pivot (being built)
    greater: Vec<Id>,
    /// Current index being compared
    current_idx: usize,
    /// Start position in final sorted array
    start_pos: usize,
}

impl QuickSortStrategy {
    pub fn new(items: Vec<Id>) -> Self {
        let mut strategy = Self {
            items: items.clone(),
            comparisons: HashMap::new(),
            state: QuickSortState {
                partition_stack: Vec::new(),
                sorted: Vec::new(),
                completed: false,
            },
        };

        // Initialize quicksort
        if items.is_empty() {
            strategy.state.completed = true;
        } else if items.len() == 1 {
            strategy.state.sorted = items.clone();
            strategy.state.completed = true;
        } else {
            strategy.initialize_quicksort();
        }

        strategy
    }

    fn initialize_quicksort(&mut self) {
        // Start with the full list
        let op = PartitionOp {
            items: self.items.clone(),
            pivot_idx: self.items.len() / 2, // Middle element as pivot
            less: Vec::new(),
            greater: Vec::new(),
            current_idx: 0,
            start_pos: 0,
        };
        self.state.partition_stack.push(op);
    }

    fn get_comparison(&self, a: &Id, b: &Id) -> Option<Id> {
        let key1 = (a.to_string(), b.to_string());
        let key2 = (b.to_string(), a.to_string());

        self.comparisons
            .get(&key1)
            .or_else(|| self.comparisons.get(&key2))
            .map(|s| Id::from(s.as_str()))
    }

    fn process_partition(&mut self) -> bool {
        if self.state.partition_stack.is_empty() {
            return false;
        }

        let op_idx = self.state.partition_stack.len() - 1;

        // Check if we've compared all items (excluding pivot)
        let needs_processing = {
            let op = &self.state.partition_stack[op_idx];
            let mut count = 0;
            for i in 0..op.items.len() {
                if i != op.pivot_idx {
                    count += 1;
                }
            }
            op.current_idx >= op.items.len() || (op.less.len() + op.greater.len()) >= count
        };

        if needs_processing {
            // Partition complete, create sub-partitions
            let op = self.state.partition_stack.pop().unwrap();
            let pivot = &op.items[op.pivot_idx];

            // Ensure sorted array is large enough
            let total_size = op.less.len() + 1 + op.greater.len();
            if self.state.sorted.len() < op.start_pos + total_size {
                self.state
                    .sorted
                    .resize(op.start_pos + total_size, Id::default());
            }

            // Place less items
            for (i, item) in op.less.iter().enumerate() {
                self.state.sorted[op.start_pos + i] = item.clone();
            }

            // Place pivot
            let pivot_pos = op.start_pos + op.less.len();
            self.state.sorted[pivot_pos] = pivot.clone();

            // Place greater items
            for (i, item) in op.greater.iter().enumerate() {
                self.state.sorted[pivot_pos + 1 + i] = item.clone();
            }

            // Push sub-partitions onto stack (greater first, so less is processed first)
            if op.greater.len() > 1 {
                let greater_op = PartitionOp {
                    items: op.greater.clone(),
                    pivot_idx: op.greater.len() / 2,
                    less: Vec::new(),
                    greater: Vec::new(),
                    current_idx: 0,
                    start_pos: pivot_pos + 1,
                };
                self.state.partition_stack.push(greater_op);
            }

            if op.less.len() > 1 {
                let less_op = PartitionOp {
                    items: op.less.clone(),
                    pivot_idx: op.less.len() / 2,
                    less: Vec::new(),
                    greater: Vec::new(),
                    current_idx: 0,
                    start_pos: op.start_pos,
                };
                self.state.partition_stack.push(less_op);
            }

            return true;
        }

        false
    }
}

impl RankStrategy for QuickSortStrategy {
    fn name(&self) -> &'static str {
        "quicksort"
    }

    fn compare(&mut self, _a: &Item, _b: &Item, winner_id: &Id) -> Result<()> {
        if self.state.partition_stack.is_empty() {
            return Ok(());
        }

        let op_idx = self.state.partition_stack.len() - 1;
        
        // First, process any cached comparisons we skipped over
        loop {
            let (current_idx, pivot_idx, items_len) = {
                let op = &self.state.partition_stack[op_idx];
                (op.current_idx, op.pivot_idx, op.items.len())
            };
            
            if current_idx >= items_len {
                break;
            }

            // Skip pivot itself
            if current_idx == pivot_idx {
                self.state.partition_stack[op_idx].current_idx += 1;
                continue;
            }

            let (current, pivot) = {
                let op = &self.state.partition_stack[op_idx];
                (op.items[current_idx].clone(), op.items[pivot_idx].clone())
            };

            // Check if we have a cached comparison for this item
            if let Some(cached_winner) = self.get_comparison(&current, &pivot) {
                let op = &mut self.state.partition_stack[op_idx];
                // Add to appropriate partition based on cached result
                if cached_winner == current {
                    op.less.push(current);
                } else {
                    op.greater.push(current);
                }
                op.current_idx += 1;
            } else {
                // This is the item we're comparing now
                break;
            }
        }

        let op = &mut self.state.partition_stack[op_idx];
        
        if op.current_idx >= op.items.len() {
            self.process_partition();
            if self.state.partition_stack.is_empty() {
                self.state.completed = true;
            }
            return Ok(());
        }

        let current = op.items[op.current_idx].clone();
        let pivot = op.items[op.pivot_idx].clone();

        // Record comparison
        let key = (current.to_string(), pivot.to_string());
        self.comparisons.insert(key, winner_id.to_string());

        // Add to appropriate partition
        if winner_id == &current {
            // Current is better (less) than pivot
            op.less.push(current);
        } else {
            // Pivot is better, current goes to greater
            op.greater.push(current);
        }

        op.current_idx += 1;

        // Check if partition is complete
        if op.current_idx >= op.items.len()
            || (op.less.len() + op.greater.len()) >= (op.items.len() - 1)
        {
            self.process_partition();

            // Check if all partitions are done
            if self.state.partition_stack.is_empty() {
                self.state.completed = true;
            }
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<RankResult> {
        if !self.state.completed {
            return Err(crate::Error::Other(
                "Ranking not complete. Continue comparing items.".to_string(),
            ));
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
        if self.state.completed || self.state.partition_stack.is_empty() {
            return None;
        }

        let op = self.state.partition_stack.last()?;

        // Find the next item that needs comparison
        let mut idx = op.current_idx;
        
        while idx < op.items.len() {
            // Skip pivot itself
            if idx == op.pivot_idx {
                idx += 1;
                continue;
            }

            let current = &op.items[idx];
            let pivot = &op.items[op.pivot_idx];

            // Check if we already have this comparison
            if self.get_comparison(current, pivot).is_none() {
                // Found an item that needs comparison
                return Some((current.clone(), pivot.clone()));
            }
            
            // This item has a cached comparison, skip it
            idx += 1;
        }

        // All items in this partition have been compared
        None
    }

    fn is_complete(&self) -> bool {
        self.state.completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Item;

    fn create_test_items(count: usize) -> Vec<Item> {
        (0..count)
            .map(|i| Item {
                id: Id::from(format!("i{}", i)),
                value: format!("Item {}", i),
                created: chrono::Utc::now(),
            })
            .collect()
    }

    #[test]
    fn test_quicksort_strategy_creation() {
        let items = create_test_items(5);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let strategy = QuickSortStrategy::new(ids);

        assert_eq!(strategy.name(), "quicksort");
        assert!(!strategy.is_complete());
    }

    #[test]
    fn test_empty_list() {
        let mut strategy = QuickSortStrategy::new(vec![]);
        assert!(strategy.is_complete());
        assert!(strategy.finalize().is_ok());
    }

    #[test]
    fn test_single_item() {
        let items = create_test_items(1);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let mut strategy = QuickSortStrategy::new(ids.clone());

        assert!(strategy.is_complete());
        let result = strategy.finalize().unwrap();
        assert_eq!(result.order.unwrap(), ids);
    }

    #[test]
    fn test_two_items() {
        let items = create_test_items(2);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let mut strategy = QuickSortStrategy::new(ids.clone());

        // Should need one comparison
        let (_a, _b) = strategy.next_comparison().unwrap();

        // Choose first item as winner
        strategy.compare(&items[0], &items[1], &ids[0]).unwrap();

        assert!(strategy.is_complete());
        let result = strategy.finalize().unwrap();
        assert_eq!(result.order.unwrap(), vec![ids[0].clone(), ids[1].clone()]);
    }

    #[test]
    fn test_three_items() {
        let items = create_test_items(3);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let mut strategy = QuickSortStrategy::new(ids.clone());

        // Perform comparisons until complete
        while let Some((a, b)) = strategy.next_comparison() {
            // Always prefer lower index (simulates consistent preference)
            let winner = if a.as_str() < b.as_str() { &a } else { &b };
            let item_a = items.iter().find(|i| i.id == a).unwrap();
            let item_b = items.iter().find(|i| i.id == b).unwrap();
            strategy.compare(item_a, item_b, winner).unwrap();
        }

        assert!(strategy.is_complete());
        let result = strategy.finalize().unwrap();
        assert_eq!(result.order.unwrap().len(), 3);
    }

    #[test]
    fn test_serialize_deserialize() {
        let items = create_test_items(4);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let mut strategy = QuickSortStrategy::new(ids.clone());

        // Do one comparison
        if let Some((a, b)) = strategy.next_comparison() {
            let item_a = items.iter().find(|i| i.id == a).unwrap();
            let item_b = items.iter().find(|i| i.id == b).unwrap();
            strategy.compare(item_a, item_b, &a).unwrap();
        }

        // Serialize
        let state = strategy.serialize_state().unwrap();

        // Create new strategy and deserialize
        let mut new_strategy = QuickSortStrategy::new(ids.clone());
        new_strategy.deserialize_state(state).unwrap();

        // Should have same state
        assert_eq!(strategy.is_complete(), new_strategy.is_complete());
    }

    #[test]
    fn test_finalize_before_complete() {
        let items = create_test_items(3);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let mut strategy = QuickSortStrategy::new(ids);

        // Try to finalize before completing
        assert!(strategy.finalize().is_err());
    }

    #[test]
    fn test_no_duplicate_comparisons() {
        let items = create_test_items(9);
        let ids: Vec<Id> = items.iter().map(|item| item.id.clone()).collect();
        let mut strategy = QuickSortStrategy::new(ids.clone());

        let mut comparisons = Vec::new();

        // Perform comparisons until complete
        while let Some((a, b)) = strategy.next_comparison() {
            // Check for duplicates
            let pair1 = (a.to_string(), b.to_string());
            let pair2 = (b.to_string(), a.to_string());
            
            for (prev_a, prev_b) in &comparisons {
                assert!(
                    !(prev_a == &pair1.0 && prev_b == &pair1.1) && 
                    !(prev_a == &pair2.0 && prev_b == &pair2.1),
                    "Duplicate comparison found: {:?} vs {:?}", a, b
                );
            }
            
            comparisons.push(pair1);

            // Always prefer lower index (simulates consistent preference)
            let winner = if a.as_str() < b.as_str() { &a } else { &b };
            let item_a = items.iter().find(|i| i.id == a).unwrap();
            let item_b = items.iter().find(|i| i.id == b).unwrap();
            strategy.compare(item_a, item_b, winner).unwrap();
        }

        assert!(strategy.is_complete());
        println!("Total comparisons for 9 items: {}", comparisons.len());
        
        // QuickSort should use significantly fewer than n*(n-1)/2 comparisons
        // For 9 items, worst case is 36 comparisons, but we should do much better
        assert!(comparisons.len() < 30, "Too many comparisons: {}", comparisons.len());
    }
}
