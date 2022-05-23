impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counter = vec![0u16; 26];
        for c in s.chars() {
            counter[(c as u8 - 97) as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if counter[(c as u8 - 97) as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
