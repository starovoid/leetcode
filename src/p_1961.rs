impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut concat = String::new();
        for w in words.iter() {
            concat.push_str(w);
            if concat == s {
                return true;
            }
            if concat.len() > s.len() {
                return false;
            }
        }
        false
    }
}
