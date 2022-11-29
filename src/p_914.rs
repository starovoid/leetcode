use std::collections::HashMap;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a > 0 && b > 0 {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    a + b
}

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counter: HashMap<i32, usize> = HashMap::new();
        for i in deck.into_iter() {
            *counter.entry(i).or_insert(0) += 1;
        }
        let m = counter.values().map(|x| *x).max().unwrap();
        counter.into_values().fold(m, |acc, i| gcd(acc, i)) > 1
    }
}
