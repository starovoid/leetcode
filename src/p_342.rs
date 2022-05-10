impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        while n >= 4 && n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}
