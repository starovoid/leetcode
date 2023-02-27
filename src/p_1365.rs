use std::collections::HashMap;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        let mut ans = Vec::with_capacity(nums.len());
        let mut mem = HashMap::new();

        for (i, x) in sorted_nums.into_iter().enumerate() {
            mem.entry(x).or_insert(i);
        }

        for x in nums.iter() {
            ans.push(*mem.get(x).unwrap() as i32);
        }

        ans
    }
}
