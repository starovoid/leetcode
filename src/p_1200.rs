impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let mut min = i32::MAX;
        let mut ans = Vec::new();

        for i in 0..arr.len() - 1 {
            let d = (arr[i+1] - arr[i]).abs();
            if d < min {
                ans.clear();
                min = d;
            }
            if d == min {
                ans.push(vec![arr[i], arr[i+1]]);
            }
        }

        ans
    }
}
