impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        arr.into_iter().enumerate().map(|(i, x)| (x as f64) * ((i as f64 + 1.0) * (n as f64 - i as f64) / 2.0).ceil()).sum::<f64>() as i32
    }
}
