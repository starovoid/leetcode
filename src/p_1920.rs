impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        (0..n as usize).for_each(|i| nums[i] += (nums[nums[i] as usize] % n) * n);
        (0..n as usize).for_each(|i| nums[i] /= n);
        nums
    }
}
