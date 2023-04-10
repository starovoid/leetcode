impl Solution {
    pub fn make_good(s: String) -> String {
        let mut ans = vec![s.chars().next().unwrap()];
        for c in s.chars().skip(1) {
            if let Some(p) = ans.pop() {
                if c == p || c.to_ascii_lowercase() != p && c.to_ascii_uppercase() != p {
                    ans.push(p);
                    ans.push(c);
                }
            } else {
                ans.push(c);
            }
        }
        ans.into_iter().collect()
    }
}
