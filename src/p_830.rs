impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut chars = s.chars().chain("_".chars());
        
        let (mut i, mut j) = (0, 0);
        let mut p = chars.next().unwrap();
        while let Some(c) = chars.next() {
            if c == p {
                j += 1;
            } else {
                if j - i >= 2 {
                    ans.push(vec![i, j]);
                }
                i = j + 1;
                j = i;
                p = c;
            }
        }

        ans
    }
}
