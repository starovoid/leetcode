impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] != 1 {
                i += 1;
                continue;
            }
            let mut j = i;
            while j < nums.len() && nums[j] == 1 {
                j += 1;
            }
            ans = ans.max(j - i);
            i = j;
        }
        ans as i32
    }
}
