impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num <= 1 {
            return true;
        }
        
        let mut low = 0 as i64;
        let mut rem = num as i64;
        
        while rem > 1 {
            let half = rem / 2;
            let mid = low + half;
            if mid * mid <= num as i64 {
                low = mid;
            }
            rem -= half;
        }
        
        low * low == num as i64
    }
}
