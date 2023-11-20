impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num % 10 != 0 || num == 0
    }
}
