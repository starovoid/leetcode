impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 0..nums.len()-1 {
            if nums[i + 1] < nums[i] {
                count += 1;
            }
        }
        if count > 0 && nums[nums.len() - 1] > nums[0] {
            count += 1;
        }

        count < 2
    }
}
