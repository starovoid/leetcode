impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut s = s.split(" ").collect::<Vec<&str>>();
        s.sort_by_key(|a| a.as_bytes()[a.len() - 1] - b'0');
        s.iter()
            .map(|w| String::from_utf8(w[..w.len() - 1].into()).unwrap())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
