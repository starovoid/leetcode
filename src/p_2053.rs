use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut counter = HashMap::new();
        for s in arr.iter() {
            *counter.entry(s).or_insert(0) += 1;
        }

        let mut distinct = vec![];
        for s in arr.iter() {
            if counter.get(&s) == Some(&1) {
                distinct.push(s);
            }
        }

        if distinct.len() >= k as usize { distinct[k as usize - 1].to_owned() } else { String::new() }
    }
}
