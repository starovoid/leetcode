impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut x: Vec<u8>;
        let mut y: Vec<u8>;
        
        if num1.len() >= num2.len() {
            x = num1.chars().rev().map(|c| (c as u8 - b'0')).collect();
            y = num2.chars().rev().map(|c| (c as u8 - b'0')).collect();
        } else {
            x = num2.chars().rev().map(|c| (c as u8 - b'0')).collect();
            y = num1.chars().rev().map(|c| (c as u8 - b'0')).collect();
        }
        
        let mut t = 0;
        for i in 0..y.len() {
            let old_t = t;
            t = (x[i] + y[i] + t) / 10;
            x[i] = (x[i] + y[i] + old_t) % 10;
        }
        
        let mut i = y.len();
        while t != 0 && i < x.len() {
            let old_t = t;
            t = (x[i] + t) / 10;
            x[i] = (x[i] + old_t) % 10;
            i += 1;
        }
        
        if t == 1 {
            x.push(1);
        }
        
        x.into_iter().rev().map(|d| (d + b'0') as char).collect()
    }
}
