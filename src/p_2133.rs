impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        for row in matrix.iter() {
            let mut present = vec![false; n];
            for &x in row {
                present[x as usize - 1] = true;
            }
            if !present.iter().all(|&x| x) {
                return false;
            }
        }

        for j in 0..n {
            let mut present = vec![false; n];
            for i in 0..n {
                present[matrix[i][j] as usize - 1] = true;
            }
            if !present.iter().all(|&x| x) {
                return false;
            }
        }

        true
    }
}
