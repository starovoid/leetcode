impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, x) in nums.into_iter().enumerate() {
            if i as i32 % 10 == x {
                return i as i32;
            }
        }
        -1
    }
}
