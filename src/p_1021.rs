impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut cnt = 1;
        let mut i = 1;
        let mut ans = String::new();
        while i < s.len() {
            if s[i] == '(' {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if cnt != 0 {
                ans.push(s[i]);
            } else {
                cnt = 1;
                i += 1;
            }
            i += 1;
        }
        ans
    }
}
