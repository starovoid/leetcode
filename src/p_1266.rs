mpl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..points.len()-1 {
            ans += dist(&points[i], &points[i+1]);
        }
        ans
    }
}

fn dist(p1: &[i32], p2: &[i32]) -> i32 {
    let dx = (p1[0] - p2[0]).abs();
    let dy = (p1[1] - p2[1]).abs();
    dx.max(dy)
}
