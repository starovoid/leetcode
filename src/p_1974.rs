impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let word = word.into_bytes();
        let mut s = (word[0] as i32 - b'a' as i32).min(26 - word[0] as i32 + b'a' as i32);
        for i in 1..word.len() {
            s += (word[i] as i32 - word[i-1] as i32).abs()
                .min(word[i] as i32 + 26 - word[i-1] as i32)
                .min(word[i-1] as i32 + 26 - word[i] as i32);
        }   
        s + word.len() as i32
    }
}
