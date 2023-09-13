impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        for (i, c) in num.bytes().enumerate().rev() {
            if (c as u8 - b'0') % 2 == 1 {
                return num.chars().take(i + 1).collect::<String>()
            }
        }
        String::new()
    }
}
