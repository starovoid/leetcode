use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut used = HashSet::with_capacity(nums.len() / 2 + 1);
        for x in nums.into_iter() {
            if !used.insert(x) {
                return x;
            }
        }
        -1
    }
}
