impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.replace("-", "").to_uppercase();
        let t = s.len() % (k as usize);
        
        let mut ans = String::new();
        let mut chars = s.chars();
        
        for _ in 0..t {
            ans.push(chars.next().unwrap());
        }
        if t != 0 && s.len() > t {
            ans.push('-');
        }
        
        for i in 0..(s.len()-t) as i32 {
            if i > 0 && i % k == 0 {
                ans.push('-');
            }
            ans.push(chars.next().unwrap());
        }
        
        ans
    }
}
