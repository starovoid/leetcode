impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut start = i;
        while i < nums.len() - 1 {
            while i < nums.len() - 1 && nums[i] < nums[i + 1] {
                i += 1;
            }
            ans = ans.max(i - start);
            start = i + 1;
            i += 1;
        }
        ans as i32 + 1
    }
}
