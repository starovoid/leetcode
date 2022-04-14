use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut used = HashSet::new();
        let mut n = n;
        
        while used.insert(n) {
            n = n.to_string()
                .chars()
                .map(|c| (c as i32 - 48) * (c as i32 - 48))
                .sum();
        }
        
        n == 1
    }
}
