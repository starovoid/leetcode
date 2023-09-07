impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let s = s.as_bytes();
        let mut count = 0;
        for i in 0..s.len()-2 {
            if s[i] != s[i+1] && s[i] != s[i+2] && s[i+1] != s[i+2] {
                count += 1;
            }
        }
        count
    }
}
