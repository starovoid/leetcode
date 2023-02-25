impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|x, y| x.count_ones().cmp(&y.count_ones()).then(x.cmp(&y)));
        arr
    }
}
