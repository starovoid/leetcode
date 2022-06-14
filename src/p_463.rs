impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = grid.len();
        let mut col = grid[0].len();
        
        let mut p = 0i32;
        
        for i in 1..row-1 {
            for j in 0..col {
                if grid[i][j] == 1 {
                    p += 4 - grid[i-1][j] - grid[i+1][j] 
                           - grid[i].get(j-1).unwrap_or(&0) - grid[i].get(j+1).unwrap_or(&0);
                }
            }
        }
        
        for j in 0..col {
            if grid[0][j] == 1 {
                p += 4 - grid[0].get(j-1).unwrap_or(&0) 
                       - grid[0].get(j+1).unwrap_or(&0) - (if row > 1 { grid[1][j] } else { 0 });
            }
            if row > 1 && grid[row-1][j] == 1 {
                p += 4 - grid[row-1].get(j-1).unwrap_or(&0) - grid[row-1].get(j+1).unwrap_or(&0) 
                       - grid[row-2][j];
            }
        }
        
        p
    }
}
