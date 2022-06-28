impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let neg = num < 0;
        let mut num = num.abs();
        let mut s = String::new();
        
        if num == 0 {
            s.push('0');
        }
        
        while num > 0 {
            s.push(((num % 7) as u8 + b'0') as char);
            num /= 7;
        }
        
        if neg {
            s.push('-');
        }
        
        s.chars().rev().collect()
    }
}
