use std::collections::HashMap;

impl Solution {
    pub fn count_balls(low: i32, high: i32) -> i32 {
        let mut map = HashMap::<i32, i32>::new();
        for mut x in low..=high {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            *map.entry(s).or_insert(0) += 1;
        }
        *map.values().max().unwrap()
    }
}
