impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        (0..accounts.len()).map(|i| (0..accounts[0].len()).map(|j| accounts[i][j]).sum::<i32>()).max().unwrap()
    }
}
