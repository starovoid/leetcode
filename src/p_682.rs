impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record: Vec<i32> = Vec::new();
        for op in operations.into_iter() {
            match op.as_str() {
                "+" => record.push(record[record.len() - 2] + record[record.len() - 1]),
                "D" => record.push(record[record.len() - 1] * 2),
                "C" => { record.pop(); },
                x => record.push(x.parse::<i32>().unwrap()),
            }
        }
        record.into_iter().sum()
    }
}
