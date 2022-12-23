use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len() == 0 {
            if n == 1 {
                return 1;
            } else {
                return -1;
            }
        }
        let mut trust_count: HashMap<i32, i32> = HashMap::new();
        for rec in trust.iter() {
            *trust_count.entry(rec[1]).or_insert(0) += 1;
        }
        (1..=n).filter(|i| trust_count.get(i) == Some(&(n - 1)))
            .filter(|i| !trust.iter().any(|d| d[0] == *i))
            .next()
            .unwrap_or(-1)
    }
}
