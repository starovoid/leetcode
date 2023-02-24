impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut count = 0;
        let mut row = 0;
        let mut col = n;
        while row < m && col > 0 {
            col -= 1;
            while row < m && grid[row][col] >= 0 {
                row += 1;
            }
            count += m - row;
        }

        count as i32
    }
}
