use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for x in arr {
            *counter.entry(x).or_insert(0) += 1;
        }
        counter.into_iter().filter(|(k, v)| k == v).map(|(k, _)| k).max().unwrap_or(-1)
    }
}
