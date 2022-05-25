impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut schars = s.chars();
        let mut tchars = t.chars();
        
        while let Some(sc) = schars.next() {
            let mut m = false;
            while let Some(tc) = tchars.next() {
                if tc == sc {
                    m = true;
                    break;
                }
            }
            if !m {
                return false;
            }
        }
        
        schars.next().is_none()
    }
}
