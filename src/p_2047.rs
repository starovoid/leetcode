impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut ans = 0;
        sentence.trim()
            .split_ascii_whitespace()
            .filter(|word| word.chars().all(|c| 'a' <= c && c <= 'z' || "!.,-".contains(c)))
            .filter(|word| {
                let h = word.matches('-').count();
                if h > 1 {
                    false
                } else if h == 1 {
                    let idx = word.find('-').unwrap();
                    idx > 0 && idx < word.len() - 1 && word.chars().nth(idx + 1).filter(|&c| 'a' <= c && c <= 'z').is_some()
                } else {
                    true
                }
            })
            .filter(|word| {
                println!("{word}");
                let mut pi = word.chars()
                    .enumerate()
                    .filter(|(i, c)| *c == '!' || *c == '.' || *c == ',')
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();
                if pi.len() > 1 {
                    false
                } else if pi.len() == 1 {
                    pi[0] == word.len() - 1
                } else {
                    true
                }
            })
            .count()
            as i32
    }
}
