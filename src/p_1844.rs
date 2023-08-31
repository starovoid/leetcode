impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut s = s.into_bytes();
        for i in (0..s.len()-1).step_by(2) {
            s[i + 1] = s[i] + s[i + 1] - b'0';
        }
        String::from_utf8(s).unwrap()
    }
}
