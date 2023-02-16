impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 0..nums.len()/2 {
            ans.extend(std::iter::repeat(nums[2*i + 1]).take(nums[2*i] as usize));
        }
        ans
    }
}
