use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map1: HashMap<char, char> = HashMap::new();
        let mut map2: HashMap<char, char> = HashMap::new();
        for (c1, c2) in s.chars().zip(t.chars()) {
            if *map1.entry(c1).or_insert(c2) != c2 || *map2.entry(c2).or_insert(c1) != c1 {
                return false;
            }
        }
        true
    }
}
