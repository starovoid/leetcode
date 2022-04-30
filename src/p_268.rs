impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let size = nums.len() as i32;
        let mut sum = size * (size + 1) / 2;
        for x in nums.into_iter() {
            sum -= x;
        }
        sum
    }
}
