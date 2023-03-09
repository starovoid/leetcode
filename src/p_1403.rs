impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }
        nums.sort_unstable();
        for i in (0..nums.len()).rev() {
            let left = &nums[0..i];
            let right = &nums[i..nums.len()];
            if left.iter().sum::<i32>() < right.iter().sum::<i32>() {
                let mut ans = right.to_owned();
                ans.sort_unstable_by(|x, y| y.cmp(x));
                return ans;
            }
        }
        vec![]
    }
}
