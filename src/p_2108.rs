impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for w in words.into_iter() {
            if is_palindromic(w.as_bytes()) {
                return w;
            }
        }
        String::new()
    }
}

fn is_palindromic(s: &[u8]) -> bool {
    for i in 0..s.len()/2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}
