impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        for i in 0..4 {
            if mat == target {
                return true;
            }
            rotate(&mut mat);
        }
        false
    }
}

fn rotate(mat: &mut Vec<Vec<i32>>) {
    let n = mat.len();
    for i in 0..n {
        for j in 0..i {
            let t = mat[i][j];
            mat[i][j] = mat[j][i];
            mat[j][i] = t;
        }
    }

    for i in 0..n {
        for j in 0..n/2 {
            let t = mat[i][j];
            mat[i][j] = mat[i][n-j-1];
            mat[i][n-j-1] = t;
        }
    }
}
