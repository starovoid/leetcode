impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counter = [0; 26];

        for w in words.iter() {
            for c in w.chars() {
                counter[(c as u8 - b'a') as usize] += 1;
            }
        }

        counter.iter().all(|cnt| cnt % words.len() == 0)
    }
}
