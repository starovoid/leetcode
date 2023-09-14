impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut removed = false;
        let mut prev = nums[0];
        
        for i in 1..nums.len() {
            if nums[i] <= prev {
                if removed {
                    return false;
                }
                removed = true;
                if i == 1 || nums[i] > nums[i - 2] {
                    prev = nums[i];
                }
            } else {
                prev = nums[i];
            }
        }

        true
    }
}
