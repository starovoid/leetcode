impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut bits = 0;
        for x in nums.iter() {
            bits |= x;
        }
        bits * (1 << nums.len()) / 2
    }
}
