impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let alph: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let words: Vec<String> = words.into_iter()
            .map(|w| w.chars().map(|c| alph[order.find(c).unwrap()]).collect())
            .collect();
        for i in 0..words.len()-1 {
            if words[i] > words[i+1] {
                return false;
            }
        }
        true
    }
}
