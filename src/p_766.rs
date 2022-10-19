impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for col in 0..matrix[0].len() {
            let val = matrix[0][col];
            let mut i = 1;
            let mut j = col + 1;
            while i < matrix.len() && j < matrix[0].len() {
                if matrix[i][j] != val {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }
        for row in 0..matrix.len() {
            let mut i = row + 1;
            let mut j = 1;
            let val = matrix[row][0];
            while i < matrix.len() && j < matrix[0].len() {
                if matrix[i][j] != val {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }
        true
    }
}
