impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans: Vec<char> = Vec::new();
        let mut cnt = 0;
        for c in s.chars() {
            if ans.last() != Some(&c) {
                ans.push(c);
                cnt = 1;
            } else if cnt < 2 {
                ans.push(c);
                cnt += 1;
            }
        }
        ans.into_iter().collect()
    }
}
