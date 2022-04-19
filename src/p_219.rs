impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if j - i > k as usize {
                    break;
                }
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}
