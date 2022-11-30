impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut stack = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_alphabetic() {
                stack.push(c);
            }
        }
        let mut ans = String::new();
        for c in s.chars() {
            if c.is_alphabetic() {
                ans.push(stack.pop().unwrap());
            } else {
                ans.push(c);
            }
        }
        ans
    }
}
