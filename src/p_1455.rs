impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence.split_whitespace()
            .enumerate()
            .filter(|(i, word)| word.starts_with(&search_word))
            .map(|(i, _)| i as i32 + 1)
            .next()
            .unwrap_or(-1)
    }
}
