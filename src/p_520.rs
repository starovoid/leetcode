impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.to_uppercase() == word
            || word.to_lowercase() == word
            || (word.chars().next().unwrap().is_ascii_uppercase()
                && word.chars().skip(1).all(|c| c.is_ascii_lowercase()))
    }
}
