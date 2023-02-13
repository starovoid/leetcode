impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; arr.len()];
        ans[arr.len() - 1] = -1;
        let mut max = arr[arr.len() - 1];
        for i in (0..arr.len()-1).rev() {
            ans[i] = max;
            max = max.max(arr[i]);
        }
        ans
    }
}
