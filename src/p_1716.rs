impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let k = n / 7;
        let m = n % 7;

        (2 * (k + 1) + m - 1) * m / 2 + if k > 0 {
            (56 + (k - 1) * 7) * k / 2
        } else {
            0
        }
    }
}
