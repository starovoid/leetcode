use std::collections::HashMap;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut cnt1 = HashMap::<char, u8>::new();
        let mut cnt2 = HashMap::<char, u8>::new();
        for c in s1.chars() {
            *cnt1.entry(c).or_insert(0) += 1;
        }
        for c in s2.chars() {
            *cnt2.entry(c).or_insert(0) += 1;
        }
        let d = s1.chars().zip(s2.chars()).fold(0, |acc, (c1, c2)| if c1 != c2 { acc + 1} else { acc });
        cnt1 == cnt2 && (d == 0 || d == 2)
    }
}
