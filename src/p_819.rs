use std::collections::HashMap;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut counter: HashMap<String, usize> = HashMap::new();
        for word in paragraph.to_ascii_lowercase()
                .replace(".", " ")
                .replace(",", " ")
                .replace("!", " ")
                .replace("?", " ")
                .replace(";", " ")
                .replace("'", " ")
                .split_ascii_whitespace() 
        {
            let w = word.to_string();
            if !banned.contains(&w) {
                *counter.entry(w).or_insert(0) += 1;
            }
        }
        counter.into_iter().max_by(|x, y| x.1.cmp(&y.1)).map(|(k, _)| k).unwrap()
    }
}
