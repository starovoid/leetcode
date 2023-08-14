impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut used = [false; 26];
        for c in sentence.chars() {
            used[(c as u8 - b'a') as usize] = true;
        }
        used.iter().all(|v| *v)
    }
}
