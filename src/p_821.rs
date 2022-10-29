impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut idxs: Vec<i32> = s.chars()
            .enumerate()
            .filter_map(|(i, ch)| if ch == c { Some(i as i32) } else { None })
            .collect();
        
        s.chars()
            .enumerate()
            .map(|(i, ch)| 
                if ch == c { 
                    0 
                } else {
                    idxs.iter().map(|&j| (j as i32 - i as i32).abs()).min().unwrap()
                })
            .collect()
    }
}
