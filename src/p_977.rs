impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        let mut neg = nums.len();
        for (i, x) in nums.iter().enumerate().rev() {
            if *x < 0 {
                neg = i;
                break;
            }
        }
        let mut pos = nums.len();
        for (i, x) in nums.iter().enumerate() {
            if *x >= 0 {
                pos = i;
                break;
            }
        }

        while neg >= 0 && neg < nums.len() && pos < nums.len() {
            if nums[neg].abs() < nums[pos] {
                ans.push(nums[neg].pow(2));
                neg -= 1;
            } else {
                ans.push(nums[pos].pow(2));
                pos += 1;
            }
        }

        while neg >= 0 && neg < nums.len() {
            ans.push(nums[neg].pow(2));
            neg -= 1;
        }
        while pos < nums.len() {
            ans.push(nums[pos].pow(2));
            pos += 1;
        }
        
        ans
    }
}
