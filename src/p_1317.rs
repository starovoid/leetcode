impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut k = 0;
        'outer: while k <= n/2 {
            k += 1;
            let mut a = k;
            let mut b = n - k;
            while a > 0 {
                if a % 10 == 0 {
                    continue 'outer;
                }
                a /= 10;
            }
            while b > 0 {
                if b % 10 == 0 {
                    continue 'outer;
                }
                b /= 10;
            }
            return vec![k, n-k];
        }
        vec![]
    }
}
