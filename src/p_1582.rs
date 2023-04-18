use std::collections::HashMap;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut row_ones = vec![0; mat.len()];
        let mut col_ones = vec![0; mat[0].len()];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                row_ones[i] += mat[i][j];
                col_ones[j] += mat[i][j];
            }
        }
        let mut ans = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 && row_ones[i] == 1 && col_ones[j] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
