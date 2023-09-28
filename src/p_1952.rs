impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut count = 0;
        for d in 2..=((n as f64).sqrt().ceil() as i32) {
            if n % d == 0 {
                count += 2 - if d * d == n { 1 } else { 0 };
            }
            if count > 1 {
                return false;
            }
        }

        count == 1
    }
}
