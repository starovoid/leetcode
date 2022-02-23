use std::collections::HashMap;

impl Solution {
    pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> { 
        let mut items: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, x) in nums.iter().enumerate() {
            if let Some(j) = items.get(&(target - x)) {
                return vec![i as i32, *j as i32];
            } else {
                items.insert(*x, i);
            }
        }
        unreachable!();
    }
}
