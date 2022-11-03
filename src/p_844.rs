impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut stack_s = Vec::new();
        let mut stack_t = Vec::new();
        let mut s = s.chars();
        let mut t = t.chars();
        
        while let Some(sc) = s.next() {
            if sc == '#' {
                stack_s.pop();
            } else {
                stack_s.push(sc);
            }
        }
        
         while let Some(tc) = t.next() {
            if tc == '#' {
                stack_t.pop();
            } else {
                stack_t.push(tc);
            }
        }
        
        stack_s == stack_t
    }
}
