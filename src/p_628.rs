impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        (nums[0] * nums[1] * nums[n-1]).max(nums[n-1] * nums[n-2] * nums[n-3])
    }
}
