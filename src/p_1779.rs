impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min_dist = i32::MAX;
        let mut min_idx = -1;
        for (i, p) in points.into_iter().enumerate() {
            if p[0] == x && (y - p[1]).abs() < min_dist {
                min_dist = (y - p[1]).abs();
                min_idx = i as i32;
            } else if p[1] == y && (x - p[0]).abs() < min_dist {
                min_dist = (x - p[0]).abs();
                min_idx = i as i32;
            }
        }
        min_idx
    }
}
