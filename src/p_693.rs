impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut s = n << 1;
        if n % 2 == 0 {
            s += 1;
        }
        (n & s == 0) && ((!n & !s) & (((n as usize).next_power_of_two() - 1) as i32)) == 0
    }
}
