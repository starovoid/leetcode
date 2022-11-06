impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(matrix[0].len());
        for j in 0..matrix[0].len() {
            ans.push((0..matrix.len()).map(|i| matrix[i][j]).collect());
        }
        ans
    }
}
