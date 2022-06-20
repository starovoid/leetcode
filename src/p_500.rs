use std::collections::HashSet;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let row1: HashSet<char> = "qwertyuiop".chars().collect();
        let row2: HashSet<char> = "asdfghjkl".chars().collect();
        let row3: HashSet<char> = "zxcvbnm".chars().collect();
        
        words.into_iter()
            .filter(|w| {
                let t: HashSet<char> = w.to_ascii_lowercase().chars().collect();
                t.is_subset(&row1) || t.is_subset(&row2) || t.is_subset(&row3)
            })
            .collect()
    }
}
