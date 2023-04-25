impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut ans = 0;
        for act in logs {
            if act == "../" {
                ans = 0.max(ans - 1);
            } else if act != "./" {
                ans += 1;
            }
        }
        ans
    }
}
