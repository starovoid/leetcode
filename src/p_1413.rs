impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut min_sum = sum;
        for x in nums.into_iter().skip(1) {
            sum += x;
            min_sum = min_sum.min(sum);
        }
        if min_sum > 0 {
            return 1;
        } else {
            return 1 - min_sum;
        }
    }
}
