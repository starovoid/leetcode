impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut p = 0;
        let mut total = 0;
        for t in time_series {
            if p > t {
                total += t + duration - p;
            } else {
                total += duration;
            }
            p = t + duration;
        }
        total
    }
}
