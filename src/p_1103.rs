impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut ans = vec![0; num_people as usize];
        let mut n = 1;
        loop {
            for i in 0..(num_people as usize) {
                ans[i] += n.min(candies);
                candies -= n;
                n += 1;
                if candies <= 0 {
                    return ans;
                }
            }
        }
    }
}
