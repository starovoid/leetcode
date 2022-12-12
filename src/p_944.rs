impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = strs.into_iter()
            .map(|s| s.chars().collect())
            .collect();

        let mut ans = 0;
        for j in 0..grid[0].len() {
            for i in 0..grid.len()-1 {
                if grid[i][j] > grid[i+1][j] {
                    ans += 1;
                    break;
                }
            }
        }

        ans
    }
}
