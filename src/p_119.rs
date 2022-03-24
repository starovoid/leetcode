impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = Vec::with_capacity(row_index as usize + 1);
        row.push(1);
        
        for k in 1..=row_index {
            row.push(
                ((row[k as usize - 1] as u64) 
                    * ((row_index - k + 1) as u64) 
                    / (k as u64)) as i32
            );
        }
        
        row
    }
}
