impl Solution {
    pub fn check_record(s: String) -> bool {
        s.matches('A').count() < 2 && !s.contains("LLL")
    }
}
