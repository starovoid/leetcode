impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] % 2 == 1 {
                while right > left && nums[right] % 2 == 1 {
                    right -= 1;
                }
                nums.swap(left, right);
            }
            left += 1;
        }
        nums
    }
}
