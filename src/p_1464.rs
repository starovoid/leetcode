impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut m1 = nums[0];
        let mut m2 = nums[1];
        if m1 < m2 {
            std::mem::swap(&mut m1, &mut m2);
        }
        for i in 2..nums.len() {
            if nums[i] > m1 {
                let t = m1;
                m1 = nums[i];
                m2 = t;
            } else if nums[i] > m2 {
                m2 = nums[i];
            }
        }
        (m1 - 1) * (m2 - 1)
    }
}
