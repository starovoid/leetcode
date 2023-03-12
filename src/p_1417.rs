impl Solution {
    pub fn reformat(s: String) -> String {
        let mut letters = Vec::new();
        let mut digits = Vec::new();

        for c in s.chars() {
            match c {
                '0'..='9' => digits.push(c),
                _ => letters.push(c),
            }
        }

        if (letters.len() as isize - digits.len() as isize).abs() > 1 {
            return "".to_owned();
        }

        let mut ans = String::with_capacity(s.len());
        let f = if letters.len() > digits.len() { 0 } else { 1 };
        for i in 0..s.len() {
            if i % 2 == f {
                ans.push(letters.pop().unwrap());
            } else {
                ans.push(digits.pop().unwrap());
            }
        }
        
        ans
    }
}
