impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }
        
        let mut nums = vec![0; n as usize + 1];
        nums[1] = 1;
        for i in 2..nums.len() {
            if i % 2 == 0 {
                nums[i] = nums[i / 2];
            } else {
                nums[i] = nums[i / 2] + nums[i / 2 + 1];
            }
        }
        nums.into_iter().max().unwrap()
    }
}
