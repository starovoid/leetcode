impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        for i in (1..=strs[0].len()).rev() {
            if strs.iter().all(|s| s.starts_with(&strs[0][0..i])) {
                return strs[0][0..i].to_string();
            }
        }
        String::new()
    }
}
