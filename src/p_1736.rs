impl Solution {
    pub fn maximum_time(time: String) -> String {
        let chars = time.chars().collect::<Vec<char>>();
        let [h1, h2, m1, m2] = [chars[0], chars[1], chars[3], chars[4]];
        
        let h = if (h1, h2) == ('?', '?') {
            23
        } else if h2 != '?' && h1 == '?' {
            10 + h2 as u8 - b'0' + if h2 < '4' { 10 } else { 0 }
        } else if h1 != '?' && h2 == '?' {
            10 * (h1 as u8 - b'0') + if h1 == '2' { 3 } else { 9 }
        } else {
            10 * (h1 as u8 - b'0') + h2 as u8 - b'0'
        };

        
        let m = if (m1, m2) == ('?', '?') {
            59
        } else if m2 != '?' && m1 == '?' {
            50 + m2 as u8 - b'0'
        } else if m1 != '?' && m2 == '?' {
            10 * (m1 as u8 - b'0') + 9
        } else {
            10 * (m1 as u8 - b'0') + m2 as u8 - b'0'
        };

        format!("{:02}:{:02}", h, m)
    }
}
