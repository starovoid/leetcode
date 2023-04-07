impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut shuffled = vec![' '; s.len()];
        for (i, c) in s.chars().enumerate() {
            shuffled[indices[i] as usize] = c;
        }
        shuffled.into_iter().collect()
    }
}
