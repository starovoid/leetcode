impl Solution {
    pub fn check_straight_line(crd: Vec<Vec<i32>>) -> bool {
        if crd[0][0] == crd[1][0] {
            crd.iter().all(|p| p[0] == crd[0][0])
        } else {
            let a = ((crd[0][1] - crd[1][1]) as f64) / ((crd[0][0] - crd[1][0]) as f64);
            let b = crd[0][1] as f64 - a * crd[0][0] as f64;
            crd.iter().all(|p| a * p[0] as f64 + b == p[1] as f64)
        }
    }
}
