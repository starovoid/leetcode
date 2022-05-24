impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut sum = t.chars().fold(0u32, |acc, c| acc + c as u32);
        s.chars().fold(sum, |acc, c| acc - c as u32) as u8 as char
    }
}
