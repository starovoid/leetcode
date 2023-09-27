impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut x = 0;
        for d in s.chars().map(|c| (c as u8 - b'a' + 1) as u64) {
            x += d / 10 + d % 10;
        }

        for _ in 1..k {
            let mut s = 0;
            while x != 0 {
                s += x % 10;
                x /= 10;
            }
            x = s;
        }

        x as i32
    }
}
