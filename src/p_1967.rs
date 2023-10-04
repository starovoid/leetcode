impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|s| word.contains(s)).count() as i32
    }
}
