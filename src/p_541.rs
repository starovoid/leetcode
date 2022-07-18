impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut chars: Vec<char> = s.chars().collect();
        
        for i in (0..chars.len()).step_by(2*k) {
            let mut b = k;
            if i + k > s.len() {
                b = s.len() - i;
            }
            for j in (0..b/2) {
                let tc = chars[i+j];
                chars[i+j] = chars[i+b-j-1];
                chars[i+b-j-1] = tc;
            }
        }
        
        chars.into_iter().collect()
    }
}
