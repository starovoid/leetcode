impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        triangle.push(vec![1]);
        
        for i in 1..(num_rows as usize) {
            let mut new_row: Vec<i32> = Vec::with_capacity(i);
            new_row.push(triangle[i-1][0]);
            for j in 0..i-1 {
                new_row.push(triangle[i-1][j] + triangle[i-1][j+1]);
            }
            new_row.push(triangle[i-1][i-1]);
            triangle.push(new_row);
        }
        
        triangle
    }
}
