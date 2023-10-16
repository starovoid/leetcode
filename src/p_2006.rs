use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = HashMap::new();
        for &x in nums.iter() {
            *counter.entry(x).or_insert(0) += 1;
        }
        counter.iter()
            .map(|(x, cnt)| cnt * (counter.get(&(x - k)).unwrap_or(&0) + counter.get(&(x + k)).unwrap_or(&0))).sum::<usize>() as i32 / 2
    }
}
