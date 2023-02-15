impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut res = Vec::with_capacity(s.len());
        let (mut k, mut sum_sym) = (0, 0);

        s.as_bytes().iter().rev().for_each(|&b| {
            match b {
                b'#' => k = 2,
                _ if k == 0 => sum_sym = b - b'0',
                _ => {
                    sum_sym += (b - b'0') * 10u8.pow(2 - k);
                    k -= 1;
                }
            };
            if k == 0 {
                res.push(sum_sym + b'a' - 1);
                sum_sym = 0;
            }
        });

        res.reverse();
        
        String::from_utf8(res).unwrap()
    }
}
