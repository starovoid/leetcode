impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counter = vec![0; 26];
        for c in ransom_note.chars() {
            counter[(c as u8 - 97) as usize] -= 1;
        }
        for c in magazine.chars() {
            counter[(c as u8 - 97) as usize] += 1;
        }
        ransom_note.chars().all(|c| counter[(c as u8 - 97) as usize] >= 0)
    }
}
