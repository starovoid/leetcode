impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 2;
        
        for _ in 2..=n {
            let t = b;
            b = a + b;
            a = t
        }
        
        a
    }
}
