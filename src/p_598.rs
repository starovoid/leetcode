impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.len() == 0 {
            return m * n;
        }
        ops.iter().min_by(|u, v| u[0].cmp(&v[0])).unwrap()[0] 
            * ops.iter().min_by(|u, v| u[1].cmp(&v[1])).unwrap()[1]
    }
}
