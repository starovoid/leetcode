impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let n = sequence.len();
        let max_k = n / word.len();
        for k in (1..=max_k).rev() {
            if sequence.contains(&word.repeat(k)) {
                return k as i32;
            }
        }
        0
    }
}
