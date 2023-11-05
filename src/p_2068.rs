impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut cnt1 = [0i16; 26];
        let mut cnt2 = [0i16; 26];
        for c in word1.chars() {
            cnt1[(c as u8 - b'a') as usize] += 1;
        }
        for c in word2.chars() {
            cnt2[(c as u8 - b'a') as usize] += 1;
        }
        (0..26).all(|i| (cnt1[i] - cnt2[i]).abs() <= 3)
    }
}
