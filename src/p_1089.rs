impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut idxs: Vec<usize> = arr.iter().enumerate().filter(|(i, x)| **x == 0).map(|(i, _)| i).collect();
        for (i, cnt) in idxs.into_iter().enumerate() {
            arr.insert(i + cnt, 0);
        }
        arr.truncate(n);
    }
}
