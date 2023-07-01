impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.matches("01").count() + s.matches("10").count() <= 1
    }
}
