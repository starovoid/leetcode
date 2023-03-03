impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rows = matrix.iter().map(|row| *row.iter().min().unwrap()).collect::<Vec<i32>>();
        let mut cols = (0..matrix[0].len()).map(|j| (0..matrix.len()).map(|i| matrix[i][j]).max().unwrap()).collect::<Vec<i32>>();
        let mut ans = Vec::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if rows[i] == x && cols[j] == x {
                    ans.push(x);
                }
            }
        }
        ans
    }
}
