impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = (area as f32).sqrt() as i32;
        loop {
            match area %w {
                0 => break vec![area / w, w],
                _ => w -= 1,
            }
        }
    }
}
