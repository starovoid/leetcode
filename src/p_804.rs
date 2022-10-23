use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let table = [
            ".-", "-...", "-.-.", "-..",".", "..-.",
            "--.", "....", "..", ".---",
            "-.-", ".-..", "--",
            "-.", "---", ".--.",
            "--.-", ".-.", "...", "-",
            "..-", "...-",".--",
            "-..-", "-.--","--..",
        ];
        
        let mut trans: HashSet<String> = HashSet::new();
        for w in words.into_iter() {
            trans.insert(
                w.chars()
                .map(|c| table[(c as u8 - b'a') as usize].chars())
                .flatten()
                .collect::<String>()
            );
        }
        
        trans.len() as i32
    }
}
