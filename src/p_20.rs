impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        
        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            } else {
                let open_brace = match c {
                    ')' => '(',
                    '}' => '{',
                    _ => '[',
                };
                if stack.pop() != Some(open_brace) {
                    return false;
                }
            }
        }
        
        stack.is_empty()
    }
}
