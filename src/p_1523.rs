impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2 + if high % 2 == 1 || low % 2 == 1 { 1 } else { 0 }
    }
}
