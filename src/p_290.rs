use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut words: Vec<&str> = s.split_ascii_whitespace().collect();
        if words.len() != pattern.len() {
            return false;
        }
        
        let mut map: HashMap<char, &str> = HashMap::new();
        let mut inv_map: HashMap<&str, char> = HashMap::new();
        let mut p = pattern.chars();
        let mut wd = words.iter();
        
        while let (Some(c), Some(&w)) = (p.next(), wd.next()) {
            let a = *map.entry(c).or_insert(w) != w;
            let b = *inv_map.entry(w).or_insert(c) != c;
            if a || b {
                return false;
            }
        }
        
        p.next().is_none() == wd.next().is_none()
    }
}
