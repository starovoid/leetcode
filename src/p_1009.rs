impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut p = 1;
        while p <= n {
            p *= 2;
        }
        p - 1 - n 
    }
}
