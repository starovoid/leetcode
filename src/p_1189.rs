use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in text.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        let b = *counter.get(&'b').unwrap_or(&0);
        let a = *counter.get(&'a').unwrap_or(&0);
        let l = *counter.get(&'l').unwrap_or(&0);
        let o = *counter.get(&'o').unwrap_or(&0);
        let n = *counter.get(&'n').unwrap_or(&0);

        b.min(a).min(l/2).min(o/2).min(n)
    }
}
