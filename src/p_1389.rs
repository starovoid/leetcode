impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut target = Vec::new();;
        for (i, idx) in index.into_iter().enumerate() {
            target.insert(idx as usize, nums[i]);
        }
        target
    }
}
