impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut license = vec![0; 26];
        license_plate.to_ascii_lowercase()
            .chars()
            .filter(|&c| 'a' <= c && c <= 'z')
            .for_each(|c| license[(c as u8 - 'a' as u8) as usize] += 1);
        
        let mut counter: Vec<usize> = Vec::with_capacity(26);
        words.into_iter()
            .filter(|w| {
                counter = vec![0; 26];
                w.to_ascii_lowercase()
                    .chars()
                    .for_each(|c| counter[(c as u8 - 'a' as u8) as usize] += 1);
                counter.iter().zip(license.iter()).all(|(a, b)| a >= b)
            })
            .min_by_key(|w| w.len())
            .unwrap()
    }
}
