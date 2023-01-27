use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counter: HashMap<i32, u16> = HashMap::new();
        for x in arr.into_iter() {
            *counter.entry(x).or_insert(0) += 1;
        }

        let mut occ = HashSet::new();
        for cnt in counter.into_values() {
            if !occ.insert(cnt) {
                return false;
            }
        }

       true
    }
}
