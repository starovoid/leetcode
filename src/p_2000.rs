impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut s = word.as_bytes().to_owned();
        let mut idx = 0;
        for i in 0..s.len() {
            if s[i] == ch as u8 {
                idx = i;
                break;
            }
        }
        for i in 0..idx/2 + idx%2 {
            s.swap(i, idx - i);
        }
        String::from_utf8(s).unwrap()
    }
}
