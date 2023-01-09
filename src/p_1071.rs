impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        for i in (1..=str1.len().min(str2.len())).rev() {
            if str1.len() % i == 0 && str2.len() % i == 0 {
                let s = &str1[0..i];
                if s.repeat(str1.len()/i) == str1 && s.repeat(str2.len()/i) == str2 {
                    return s.to_owned();
                }
            }
        }
        String::new()
    }
}
