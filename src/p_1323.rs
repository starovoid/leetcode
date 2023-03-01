impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let s = num.to_string();
        for (i, c) in s.chars().enumerate() {
            if c == '6' {
                return format!("{}9{}", &s[..i], &s[i+1..]).parse::<i32>().unwrap();
            }
        }
        num
    }
}
