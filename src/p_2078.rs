impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        for d in (0..colors.len()).rev() {
            for i in 0..colors.len()-d {
                if colors[i] != colors[i + d] {
                    return d as i32;
                }
            }
        }
        -1
    }
}
