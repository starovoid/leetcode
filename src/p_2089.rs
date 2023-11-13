impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut min_cnt = nums.iter().enumerate().filter(|(i, &x)| x < target).map(|(i, _)| i as i32).count() as i32;
        let mut target_cnt = nums.iter().filter(|&x| *x == target).count() as i32;
        (min_cnt..min_cnt + target_cnt).collect()
    }
}
