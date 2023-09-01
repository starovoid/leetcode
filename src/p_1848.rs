impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut i = 0;
        let mut dist = i32::MAX;
        for (i, &x) in nums.iter().enumerate() {
            if x == target && (start - i as i32).abs() < dist {
                dist = (start - i as i32).abs();
            }
        }
        dist
    }
}
