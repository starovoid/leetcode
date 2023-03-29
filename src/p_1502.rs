impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let t = arr[1] - arr[0];
        (1..arr.len()).all(|i| arr[i] - arr[i-1] == t)
    }
}
