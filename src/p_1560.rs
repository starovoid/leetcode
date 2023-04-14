impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let a = rounds[0];
        let b = rounds[rounds.len() - 1];
        if a <= b {
            (a..=b).collect()
        } else {
            (1..=b).chain(a..=n).collect()
        }
    }
}
