impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let w = coordinates.chars().next().unwrap();
        let h = coordinates.chars().skip(1).next().unwrap();
        (w as u8 - b'a' + 1 + h as u8 - b'0') % 2 == 1
    }
}
