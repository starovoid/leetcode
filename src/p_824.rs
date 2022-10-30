impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut words: Vec<String> = sentence.split_ascii_whitespace()
            .map(|s| s.clone().to_string())
            .collect();
        
        for (i, w) in words.iter_mut().enumerate() {
            let f = w.chars().next().unwrap();
            if ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&f) {
                w.push_str("ma");
            } else {
                w.push(f);
                w.push_str("ma");
                w.remove(0);
            }
            w.push_str("a".repeat(i + 1).as_str());
        }
        
        words.join(" ")
    }
}
