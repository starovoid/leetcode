impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut ans: Vec<char> = Vec::new();
        let mut cnt = 0;
        for c in number.chars().filter(|&c| c != ' ' && c != '-') {
            if cnt == 3 {
                cnt = 0;
                ans.push('-');
            }
            ans.push(c);
            cnt += 1;
        }
        let n = ans.len();
        if ans[n - 2] == '-' {
            ans.swap(n - 2, n - 3);
        }
        ans.into_iter().collect()
    }
}
