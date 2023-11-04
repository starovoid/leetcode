use std::collections::HashSet;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        if word.len() < 5 {
            return 0;
        }

        let wb = word.as_bytes();
        let mut substr = HashSet::with_capacity(5);
        let mut ans = 0;

        for i in 0..word.len() - 4 {
            for &b in wb[i..].iter().take_while(|b| [b'a', b'e', b'i', b'o', b'u'].contains(b)) {
                substr.insert(b);
                if substr.len() == 5 {
                    ans += 1;
                }
            }
            substr.clear();
        }

        ans
    }
}
