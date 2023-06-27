impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        for len in (2..=s.len()).rev() {
            for i in 0..=(s.len() - len) {
                let mut cnt1 = [0; 26];
                let mut cnt2 = [0; 26];
                for c in (&s[i..i+len]).chars() {
                    if c >= 'a' {
                        cnt1[(c as u8 - b'a') as usize] += 1;
                    } else {
                        cnt2[(c as u8 - b'A') as usize] += 1;
                    }
                }
                println!("{:?}", &s[i..i+len]);
                if (0..26).all(|i| cnt1[i] == 0 && cnt2[i] == 0 || cnt1[i] > 0 && cnt2[i] > 0) {
                    return s[i..i+len].to_owned();
                }
            }
        }
        String::new()
    }
}
