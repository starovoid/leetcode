use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        
        let mut ans = 0;
        let mut odd = 0;
        for (k, &v) in counter.iter() {
            if v % 2 == 0 {
                ans += v;
            } else {
                ans += v - 1;
                odd = 1;
            }
        }
        ans + odd
    }
}
