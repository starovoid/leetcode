impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut perm = Vec::with_capacity(s.len() + 1);
        let mut min = 0;
        let mut max = s.len() as i32;
        let mut last = 'I';

        for (i, c) in s.chars().enumerate() {
            if c == 'I' {
                perm.push(min);
                min += 1;
            } else {
                perm.push(max);
                max -= 1;
            }
            if i == s.len() {
                last = c;
            }
        }

        if last == 'I' {
            perm.push(max);
        } else {
            perm.push(min);
        }
        
        perm
    }
}
