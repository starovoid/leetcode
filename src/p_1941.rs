impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut counter = [0u16; 26];
        for c in s.chars() {
            counter[(c as u8 - b'a') as usize] += 1;
        }
        let cnt: u16 = *counter.iter().filter(|&&x| x != 0).next().unwrap();
        counter.iter().all(|&x| x == 0 || x == cnt)
    }
}
