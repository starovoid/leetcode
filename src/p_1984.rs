impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if k == 1 {
            return 0;
        }

        nums.sort_unstable();
        let mut diff = nums[k - 1] - nums[0];

        for i in 1..=nums.len()-k {
            diff = diff.min(nums[i + k - 1] - nums[i]);
        }

        diff
    }
}
