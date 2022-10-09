impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let ones = bits
            .iter()
            .rev()
            .skip(1)
            .take_while(|b| **b == 1)
            .count();

        ones % 2 == 0
    }
}
