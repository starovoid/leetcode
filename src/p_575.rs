use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        candy_type.into_iter().collect::<HashSet<i32>>().len().min(n / 2) as i32
    }
}
