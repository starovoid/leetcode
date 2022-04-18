use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for x in nums.into_iter() {
            if !seen.insert(x) {
                return true;
            }
        }
        false
    }
}
