impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let ch = ['0', '1'];
        let mut i = 0;
        let (mut a, mut b) = (0, 0);

        s.chars().for_each(|c| {
            match c != ch[i] {
                true => a += 1,
                false => b += 1,
            }
            i = (i + 1) % 2;
        });

        a.min(b)
    }
}
