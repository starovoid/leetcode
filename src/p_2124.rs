impl Solution {
    pub fn check_string(s: String) -> bool {
        if s.len() == 1 {
            return true;
        }

        let sb = s.as_bytes();
        let bi = match sb.iter().position(|&c| c == b'b') {
            Some(i) => i,
            None => return true,
        };

        !&sb[bi..].contains(&b'a')
    }
}
