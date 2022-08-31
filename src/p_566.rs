impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if (r * c) as usize != mat.len() * mat[0].len() {
            return mat;
        }
        
        let mut mat_iter = mat.into_iter().flatten();
        let mut ans = vec![vec![0; c as usize]; r as usize];
        for i in 0..(r as usize) {
            for j in 0..(c as usize) {
                ans[i][j] = mat_iter.next().unwrap();
            }
        }
        
        ans
    }
}
