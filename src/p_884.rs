use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words: HashMap<String, usize> = HashMap::new();
        for w in s1.split_ascii_whitespace().map(|s| s.to_string()) {
            *words.entry(w).or_insert(0) += 1;
        }

        for w in s2.split_ascii_whitespace().map(|s| s.to_string()) {
            *words.entry(w).or_insert(0) += 1;
        }

        words.into_iter().filter(|(w, cnt)| *cnt == 1).map(|(w, _)| w).collect()
    }
}
