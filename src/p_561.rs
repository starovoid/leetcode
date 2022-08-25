impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.into_iter().step_by(2).sum()
    }
}
