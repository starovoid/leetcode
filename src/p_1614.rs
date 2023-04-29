impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        let mut max_depth = depth;
        for c in s.chars() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
            }
            max_depth = max_depth.max(depth);
        }
        max_depth
    }
}
