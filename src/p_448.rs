use std::collections::HashSet;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let nums: HashSet<i32> = nums.into_iter().collect();
        (1..=(n as i32)).filter(|x| !nums.contains(x)).collect()
    }
}
