impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rev().fold(0, |acc, c| {
            acc + match c {
                'M' => 1000,
                'D' => 500,
                'C' => if acc >= 500 { -100 } else { 100 },
                'L' => 50,
                'X' => if acc >= 50 { -10 } else { 10 },
                'V' => 5,
                'I' => if acc >= 5 { -1 } else { 1 },
                _ => 0,
            }
        })
    }
}
