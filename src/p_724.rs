impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = nums.iter().sum::<i32>();
        for (i, &x) in nums.iter().enumerate() {
            if x == sum {
                return i as i32;
            }
            sum -= x * 2;
        }
        -1
    }
}
