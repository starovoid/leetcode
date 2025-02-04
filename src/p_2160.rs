impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = [0; 4];
        for (i, deg) in [1, 10, 100, 1000].into_iter().enumerate() {
            digits[i] = (num / deg) % 10;
        }

        digits.sort_unstable();

        let x = digits[0] * 10 + digits[2];
        let y = digits[1] * 10 + digits[3];

        x + y
    }
}
