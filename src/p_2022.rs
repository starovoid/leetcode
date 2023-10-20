impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if n * m == original.len() as i32 {
            original.chunks(n as usize)
            .map(|items| items.to_vec())
            .collect::<Vec<Vec<i32>>>()
        } else {
            vec![]
        }
    }
}
