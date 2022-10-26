impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut width = 0;
        let mut lines = 0;
        for c in s.chars() {
            let w = widths[(c as u8 - b'a') as usize];
            if width + w > 100 {
                width = w;
                lines += 1;
            } else {
                width += w;
            }
        }
        vec![lines + if width > 0 { 1 } else { 0 }, width]
    }
}
