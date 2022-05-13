impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut positions: Vec<usize> = Vec::new();
        
        let mut vovels: Vec<char> = s.chars()
            .enumerate()
            .filter(|(i, c)| 
                match c {
                    'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => {
                        positions.push(*i);
                        true
                    },
                    _ => false,
                }
            )
            .map(|(_, c)| c)
            .collect();
        
        let mut s: Vec<char> = s.chars().collect();
        
        for pos in positions.into_iter() {
            s[pos] = vovels.pop().unwrap();
        }
        
        s.into_iter().collect()
    }
}
