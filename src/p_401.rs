impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ans = Vec::new();
        for h in 0..=11u8 {
            for m in 0..=59u8 {
                if h.count_ones() + m.count_ones() == turned_on as u32 {
                    ans.push(format!("{}:{:02}", h, m));
                }
            }
        }
        ans
    }
}
