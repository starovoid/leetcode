impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut ones = 0;
        let mut zeros = 0;
        let mut cur_num = s.chars().next().unwrap();
        let mut cur_len = 0;
        for c in s.chars() {
            if c == cur_num {
                cur_len += 1;
            } else {
                if cur_num == '1' {
                    ones = ones.max(cur_len);
                } else {
                    zeros = zeros.max(cur_len);
                }
                cur_len = 1;
                cur_num = c;
            }
        }
        if cur_num == '1' {
            ones = ones.max(cur_len);
        } else {
            zeros = zeros.max(cur_len);
        }
        ones > zeros
    }
}
