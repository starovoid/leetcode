use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut out = HashMap::<String, i32>::new();
        for mut edge in paths.into_iter() {
            out.entry(edge.pop().unwrap()).or_insert(0);
            *out.entry(edge.pop().unwrap()).or_insert(0) += 1;
        }
        out.into_iter().filter(|(city, cnt)| *cnt == 0).map(|(city, _)| city).next().unwrap()
    }
}
