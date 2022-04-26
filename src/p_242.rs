use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut comp: HashMap<char, isize> = HashMap::new();
        s.chars().for_each(|c| *comp.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *comp.entry(c).or_insert(0) -= 1);
        comp.values().all(|cnt| *cnt == 0)
    }
}
