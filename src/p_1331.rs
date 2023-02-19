use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, x) in arr.iter().enumerate() {
            pos.entry(*x).or_insert(vec![]).push(i);
        }

        let mut sorted = arr.clone();
        sorted.sort_unstable();

        let mut ans = vec![0; arr.len()];
        let mut used = HashSet::new();
        let mut rank = 1;
        for x in sorted.into_iter() {
            if !used.insert(x) {
                continue;
            }
            for &i in pos.get(&x).unwrap().iter() {
                ans[i] = rank;
            }
            rank += 1;
        }

        ans
    }
}
