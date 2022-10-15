impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_idx = 0;
        for (i, &x) in nums.iter().enumerate() {
            if x > max {
                max = x;
                max_idx = i;
            }
        }
        
        for x in nums.into_iter() {
            if x != max && max < x * 2 {
                return -1;
            }
        }
        
        max_idx as i32
    }
}
