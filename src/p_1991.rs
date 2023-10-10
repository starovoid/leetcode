impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = vec![0; nums.len()];   
        let mut right_sum = vec![0; nums.len()];

        for i in 1..nums.len() {
            left_sum[i] = left_sum[i - 1] + nums[i - 1];
        }
        for i in (0..nums.len()-1).rev() {
            right_sum[i] = right_sum[i + 1] + nums[i + 1];
        }

        for i in 0..nums.len() {
            if left_sum[i] == right_sum[i] {
                return i as i32;
            }
        }

        -1
    }
}
