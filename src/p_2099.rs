use std::collections::HashSet;

impl Solution {
    pub fn max_subsequence(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut idxs = (0..nums.len()).collect::<Vec<usize>>();
        idxs.sort_unstable_by_key(|&i| -nums[i]);
        let presents = idxs.into_iter()
            .take(k as usize)
            .collect::<HashSet<usize>>();
        nums.into_iter()
            .enumerate()
            .filter(|(i, x)| presents.contains(i))
            .map(|(_, x)| x)
            .collect()
    }
}
