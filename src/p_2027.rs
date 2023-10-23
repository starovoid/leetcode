impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut ans = 0;
        let mut chars = s.chars();
        let mut skip = 0;
        while let Some(c) = chars.next() {
            if skip != 0 {
                skip -= 1;
                continue;
            }
            if c == 'X' {
                ans += 1;
                skip = 2;
            }
        }
        ans
    }
}
