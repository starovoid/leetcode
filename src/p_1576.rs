impl Solution {
    pub fn modify_string(s: String) -> String {
        if s.len() == 1 && s.chars().next().unwrap() == '?' {
            return "a".into();
        }

        let n = s.len();
        let mut s = s.chars().collect::<Vec<_>>();
        if s[0] == '?' {
            for c in 'a'..'z' {
                if c != s[1] {
                    s[0] = c;
                }
            }
        }
        if s[n - 1] == '?' {
            for c in 'a'..'z' {
                if c != s[s.len() - 2] {
                    s[n - 1] = c;
                }
            }
        }

        for i in 1..n-1 {
            if s[i] == '?' {
                for c in 'a'..'z' {
                    if c != s[i-1] && c != s[i+1] {
                        s[i] = c;
                    }
                }
            }
        }

        s.into_iter().collect()
    }
}
