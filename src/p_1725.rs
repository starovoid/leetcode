impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let m = rectangles.iter().map(|rect| rect[0].min(rect[1])).max().unwrap();
        rectangles.into_iter().filter(|rect| rect[0].min(rect[1]) == m).count() as i32
    }
}
