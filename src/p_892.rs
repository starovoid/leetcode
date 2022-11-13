impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == 0 {
                    continue;
                }
                area += 2;
                let h = grid[i][j];
                if i == 0 {
                    area += h;
                } else {
                    area += (h - grid[i-1][j]).max(0);
                }
                if i == grid.len() - 1 {
                    area += h;
                } else {
                    area += (h - grid[i+1][j]).max(0);
                }
                if j == 0 {
                    area += h;
                } else {
                    area += (h - grid[i][j-1]).max(0);
                }if j == grid.len() - 1 {
                    area += h;
                } else {
                    area += (h - grid[i][j+1]).max(0);
                }
            }
        }
        area
    }
}
