impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (start..)
            .step_by(2)
            .take(n as usize)
            .fold(0, |acc, x| acc ^ x)
    }
}
