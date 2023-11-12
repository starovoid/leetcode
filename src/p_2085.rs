use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut cnt1 = HashMap::new();
        let mut cnt2 = HashMap::new();
        for w in &words1 {
            *cnt1.entry(w).or_insert(0) += 1;
        }
        for w in &words2 {
            *cnt2.entry(w).or_insert(0) += 1;
        }
        cnt1.into_iter().filter(|(w, c)| *c == 1 && cnt2.get(w) == Some(&1)).count() as i32
    }
}
