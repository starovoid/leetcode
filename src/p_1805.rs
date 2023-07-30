use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut s = HashSet::<String>::new();
        for n in word.chars()
                .map(|c| if c.is_ascii_digit() { c } else { ' ' })
                .collect::<String>()
                .split_ascii_whitespace()
                .map(|x| x.trim_start_matches('0').to_string()) 
        {
            s.insert(n);
        }
        s.len() as i32
    }
}
