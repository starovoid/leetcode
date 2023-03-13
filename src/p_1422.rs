impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut zeros = if &s[0..1] == "0" { 1 } else { 0 };
        let mut ones = s.chars().skip(1).filter(|c| *c == '1').count();
        let mut max_score = zeros + ones;
        for c in s.chars().skip(1).take(s.len() - 2) {
            if c == '0' {
                zeros += 1;
            } else {
                ones -= 1;
            }
            max_score = max_score.max(zeros + ones);
        }
        max_score as i32
    }
}
