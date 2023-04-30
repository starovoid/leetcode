impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let k = arr.len() / 20;
        arr.sort_unstable();
        (k..arr.len()-k).map(|i| arr[i] as f64).sum::<f64>() / (arr.len() as f64 - (k*2) as f64)
    }
}
