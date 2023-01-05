use std::collections::HashSet;

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        if points.iter().cloned().collect::<HashSet<Vec<i32>>>().len() != points.len() {
            return false;
        }
        let k = ((points[1][1] - points[0][1]) as f64) / ((points[1][0] - points[0][0]) as f64);
        let b = (points[0][1] as f64) - k * (points[0][0] as f64);

        points.iter().skip(2).any(|p|
            if k == 0.0 {
                p[1] != points[0][1]
            } else if k == f64::INFINITY || k == f64::NEG_INFINITY {
                p[0] != points[0][0] 
            } else {
                (1000.0 * (k * (p[0] as f64) + b)).round() / 1000.0 != (p[1] as f64)
            }
        )
    }
}
