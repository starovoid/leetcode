impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n  = code.len();

        if k == 0 {
            return vec![0; n];
        };

        let mut res = Vec::with_capacity(n);
        let mut start = match k.signum() {
            -1 => n as i32 + k,
            s => s,
        } as usize;
        let steps = k.abs() as usize;

        for _ in 0..n {
            let mut s = 0;
            (start..start + steps).for_each(|j| s += code[j % n]);
            res.push(s);
            start += 1;
        }
        res
    }
}
