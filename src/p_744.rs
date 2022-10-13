impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut ans = letters[0];
        for c in letters.into_iter() {
            if c > target {
                if ans <= target {
                    ans = c;
                } else if c < ans {
                    ans = c;
                }
            }
        }
        ans
    }
}
