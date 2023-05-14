impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allow = vec![false; 26];
        for c in allowed.chars() {
            allow[(c as u8 - b'a') as usize] = true;
        }
        words.into_iter().filter(|w| w.chars().all(|c| allow[(c as u8 - b'a') as usize])).count() as i32
    }
}
