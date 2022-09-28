impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if s[left] != s[right] {
                let mut i = left;
                let mut j = right - 1;
                let mut v1 = true;
                while i < j {
                    if s[i] != s[j] {
                        v1 = false;
                        break;
                    }
                    i += 1;
                    j -= 1;
                }
                let mut i = left + 1;
                let mut j = right;
                let mut v2 = true;
                while i < j {
                    if s[i] != s[j] {
                        v2 = false;
                        break;
                    }
                    i += 1;
                    j -= 1;
                }
                return v1 || v2;
            }
            left += 1;
            right -= 1;
        }
        
        true
    }
}
