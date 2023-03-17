impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut ans = 0;
        let mut s = s.chars();
        let mut len = 1;
        let mut cur = s.next().unwrap();
        while let Some(c) = s.next() {
            if c == cur {
                len += 1;
            } else {
                ans = ans.max(len);
                len = 1;
                cur = c;
            }
        }
        ans.max(len)
    }
}
