use std::collections::HashSet;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }

        let max = n * n;
        let mut count = 0;
        let sset = (1..=n).map(|x| x * x).collect::<HashSet<_>>();

        for (i, a2) in (1..n-1).map(|x| x * x).enumerate() {
            for b2 in ((i as i32)+2..n).map(|x| x * x) {
                println!("{} {}", a2, b2);
                if a2 + b2 > max {
                    break;
                }
                if sset.contains(&(a2 + b2)) {
                    count += 2;
                }
            }
        }

        count
    }
}
