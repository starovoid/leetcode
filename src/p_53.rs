impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let mut sum = 0;
        for x in nums.into_iter() {
            max = max.max(sum + x);
            sum = 0.max(sum + x);
        }
        max
    }
}
