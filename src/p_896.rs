impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut inc = true;
        let mut dec = true;
        for i in 0..nums.len() - 1 {
            if nums[i+1] < nums[i] {
                inc = false;
            }
            if nums[i+1] > nums[i] {
                dec = false;
            }
        }
        inc || dec
    }
}
