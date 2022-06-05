use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut used = HashSet::with_capacity(3);
        while heap.len() > 0 && used.len() < 3 {
            used.insert(heap.pop().unwrap());
        }
        if used.len() == 3 {
            *used.iter().min().unwrap() 
        } else {
            *used.iter().max().unwrap()
        }
    }
}
