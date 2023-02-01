impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];
        for i in indices.iter() {
            rows[i[0] as usize] += 1;
            cols[i[1] as usize] += 1;
        }

        let mut ans = 0;
        for r in rows.iter() {
            for c in cols.iter() {
                if (r + c) % 2 == 1 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
