impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max = salary.iter().max().unwrap();
        let mut min = salary.iter().min().unwrap();
        ((salary.iter().sum::<i32>() - min - max) as f64) / (salary.len() as f64 - 2.0)
    }
}
