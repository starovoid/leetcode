impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut rcnt = 0;
        let mut lcnt = 0;

        for c in s.chars() {
            if rcnt == lcnt && rcnt > 0 {
                count += 1;
                rcnt = 0;
                lcnt = 0;
            }
            if c == 'R' {
                rcnt += 1;
            } else {
                lcnt += 1;
            }
        }

        if rcnt > 0 && rcnt == lcnt {
            count += 1;
        }

        count
    }
}
