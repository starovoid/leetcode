impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let size = rows * cols;

        grid.into_iter()
            .flatten()
            .cycle()
            .skip(size - (k as usize) % size)
            .take(size)
            .collect::<Vec<_>>()
            .chunks(cols)
            .map(|row| row.to_owned())
            .collect::<Vec<_>>()
    }
}
