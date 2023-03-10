use std::collections::HashSet;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ans = HashSet::new();
        for i in 0..words.len() {
            for j in i+1..words.len() {
                if words[i].contains(&words[j]) {
                    ans.insert(words[j].clone());
                }
                if words[j].contains(&words[i]) {
                    ans.insert(words[i].clone());
                }
            }
        }
        ans.into_iter().collect()
    }
}
