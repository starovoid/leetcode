use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut gr = HashMap::new();
        for x in 1..=n {
            gr.entry(sum(x)).or_insert(vec![]).push(x);
        }
        let mut size = gr.values().map(|g| g.len()).max().unwrap();
        gr.values().filter(|g| g.len() == size).count() as i32
    }
}

fn sum(mut n: i32) -> i32 {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    s
}
