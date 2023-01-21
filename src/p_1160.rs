use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut counter: HashMap<char, u16> = HashMap::new();
        for c in chars.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        let mut ans = 0;
        for word in words.into_iter() {
            let mut cnt: HashMap<char, u16> = HashMap::new();
            for c in word.chars() {
                *cnt.entry(c).or_insert(0) += 1;
            }
            if cnt.iter().all(|(k, v)| counter.get(k).unwrap_or(&0) >= v) {
                ans += word.len();
            }
        }

        ans as i32
    }
}
