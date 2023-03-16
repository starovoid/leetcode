impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = 0;
        let mut i = 0;
        while i < nums.len() && nums[i] != 1 {
            i += 1;
        }
        last = i;
        for j in i+1..nums.len() {
            if nums[j] == 1 && j - last - 1 < k  as usize {
                return false;
            }
            if nums[j] == 1 {
                last = j;
            }
        }
        true
    }
}
