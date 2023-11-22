impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title.split_ascii_whitespace()
            .map(|word| 
                if word.len() < 3 {
                    word.to_ascii_lowercase()
                } else {
                    word.chars()
                    .take(1)
                    .map(|c| c.to_ascii_uppercase())
                    .chain(word.to_ascii_lowercase().chars().skip(1))
                    .collect::<String>()
                }
            )
            .collect::<Vec<String>>()
            .join(" ")
    }
}
