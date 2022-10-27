impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut ans = 0.0f64;
        for i in 0..points.len() {
            for j in i+1..points.len() {
                for k in j+1..points.len() {
                    let det = (points[j][0] - points[i][0]) * (points[k][1] - points[i][1])
                        - (points[k][0] - points[i][0]) * (points[j][1] - points[i][1]);
                    ans = ans.max(det.abs() as f64 / 2.0);
                }
            }
        }
        ans
    }
}
