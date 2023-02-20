impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        if is_palindrome(&s) {
            1
        } else {
            2
        }
    }
}

pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    (0..bytes.len()/2).all(|i| bytes[i] == bytes[bytes.len() - 1 - i])
}
