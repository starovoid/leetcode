impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[0]);
        let mut max_width = points[1][0] - points[0][0];
        for i in 2..points.len() {
            max_width = max_width.max(points[i][0] - points[i-1][0]);
        }
        max_width
    }
}
