use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq: HashMap<i32, u8> = HashMap::new();
        for &x in nums.iter() {
            *freq.entry(x).or_insert(0) += 1;
        }
        nums.sort_unstable_by(|x, y| freq.get(x).unwrap().cmp(freq.get(y).unwrap()).then(y.cmp(x)));
        nums
    }
}
