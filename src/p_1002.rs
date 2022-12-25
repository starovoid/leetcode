use std::collections::HashMap;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut general_cnt: HashMap<char, usize> = HashMap::new();
        for c in words[0].chars() {
            *general_cnt.entry(c).or_insert(0) += 1;
        }
        for i in 1..words.len() {
            let mut cnt: HashMap<char, usize> = HashMap::new();
            for c in words[i].chars() {
                *cnt.entry(c).or_insert(0) += 1;
            }
            for (k, v) in general_cnt.iter_mut() {
                *v = (*v).min(*cnt.get(k).unwrap_or(&0));
            }
        }
        general_cnt.into_iter()
            .filter(|(k, v)| *v > 0)
            .map(|(k, v)| std::iter::repeat(k.to_string()).take(v))
            .flatten()
            .collect()
    }
}
