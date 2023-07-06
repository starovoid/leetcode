impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                max_sum = max_sum.max(sum);
                sum = 0;
            }
            sum += nums[i];
        }
        max_sum.max(sum)
    }
}
