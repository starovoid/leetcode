impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut left = vec![None; 26];
        let mut right = vec![None; 26];
        for (i, c) in s.chars().enumerate() {
            if left[(c as u8 - b'a') as usize].is_none() {
                left[(c as u8 - b'a') as usize] = Some(i as i32);
            }
            right[(c as u8 - b'a') as usize] = Some(i as i32);
        }
        s.chars()
            .map(|c| right[(c as u8 - b'a') as usize].unwrap_or(0) - left[(c as u8 - b'a') as usize].unwrap_or(0) - 1).max().unwrap_or(-1) as i32
    }
}
