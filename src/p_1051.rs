impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.iter().cloned().collect::<Vec<i32>>();
        expected.sort_unstable();
        heights.into_iter().zip(expected.into_iter()).filter(|(x, y)| x != y).count() as i32
    }
}
