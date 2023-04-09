impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut nxt = 1;
        let mut n = 0;
        for x in arr {
            while nxt < x {
                nxt += 1;
                n += 1;
                if n == k {
                    return nxt - 1;
                }
            }
            nxt += 1;
        }
        nxt + k - n - 1
    }
}
