impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        
        let mut low = 0u64;
        let mut rem = x as u64;
        
        while rem > 1 {
            let half = rem / 2;
            let mid = low + half;
            if mid * mid <= x as u64 {
                low = mid;
            }
            rem -= half;
        }
        
        low as i32
    }
}
