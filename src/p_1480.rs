impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut ans = Vec::with_capacity(nums.len());
        for x in nums.into_iter() {
            sum += x;
            ans.push(sum);
        }
        ans
    }
}
