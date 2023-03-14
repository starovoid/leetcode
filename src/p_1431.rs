impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let m = candies.iter().cloned().max().unwrap();
        candies.into_iter().map(|x| x + extra_candies >= m).collect()
    }
}
