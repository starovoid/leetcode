impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        cost.iter()
            .rev()
            .enumerate()
            .filter_map(|(i, x)| if i % 3 == 2 { None } else { Some(x) })
            .sum()
    }
}
