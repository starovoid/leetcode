impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut max_dist = 0;
        let mut i = 0;
        while n > 0 && n % 2 == 0 {
            i += 1;
            n /= 2;
        }
        let mut pos = i;
        while n > 0 {
            if n % 2 == 1 {
                max_dist = max_dist.max(i - pos);
                pos = i;
            }
            i += 1;
            n /= 2;
        }
        max_dist
    }
}
