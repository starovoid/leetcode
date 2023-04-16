impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        (0..mat.len()).map(|i| mat[i][i] + mat[i][mat.len()-i-1]).sum::<i32>() - if mat.len() % 2 == 1 { mat[mat.len()/2][mat.len()/2] } else { 0 }
    }
}
