impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut last = 0;
        for token in s.split_ascii_whitespace() {
            if let Ok(num) = token.parse::<u8>() {
                if num <= last {
                    return false;
                }
                last = num;
            }
        }
        true
    }
}
