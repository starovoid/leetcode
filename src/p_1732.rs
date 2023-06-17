impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        gain.into_iter().fold(0, |acc, x| {
            max = max.max(acc + x);
            acc + x
        });
        max
    }
}
