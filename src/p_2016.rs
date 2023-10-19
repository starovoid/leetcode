impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mins = vec![nums[0]; n];
        let mut maxs = vec![nums[n - 1]; n];
        for (i, &x) in nums.iter().enumerate().skip(1) {
            mins[i] = mins[i - 1].min(x);
        }
        for (i, &x) in nums.iter().enumerate().rev().skip(1) {
            maxs[i] = maxs[i + 1].max(x);
        }

        let mut ans = -1;
        for i in 0..n-1 {
            ans = ans.max(maxs[i + 1] - mins[i]);
        }

        if ans == 0 { -1 } else { ans }
    }
}
