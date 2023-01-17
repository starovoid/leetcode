use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut counter: HashMap<(i32, i32), i32> = HashMap::new();
        for p in dominoes.into_iter().map(|p| (p[0], p[1])) {
            *counter.entry(p).or_insert(0) += 1;
        }
        let mut h = 0;
        let mut s = 0;
        for (k, v) in counter.iter() {
            h += v * (v - 1) / 2;
            if k.1 != k.0 {
                s += v * counter.get(&(k.1, k.0)).unwrap_or(&0);
            }
        }
        h + s / 2
    }
}
