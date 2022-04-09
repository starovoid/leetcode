impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut maj = (nums[0], 1);
        
        for &x in nums.iter().skip(1) {
            if x == maj.0 {
                maj.1 += 1;
            } else {
                maj.1 -= 1;
            }
            if maj.1 < 0 {
                maj = (x, 1);
            }
        }
        
        maj.0
    }
}
