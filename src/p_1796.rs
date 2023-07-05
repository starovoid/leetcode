impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut seen = [0; 10];
        for c in s.chars() {
            if c.is_ascii_digit() {
                seen[(c as u8 - b'0') as usize] += 1;
            }
        }
        let mut n = 1;
        for (i, &cnt) in seen.iter().enumerate().rev() {
            if cnt > 0 && n > 1 {
                return i as i32;
            } else if cnt > 0 {
                n += 1;
            }
        }
        -1
    }
}
