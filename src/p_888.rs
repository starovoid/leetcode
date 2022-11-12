impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sa: i32 = alice_sizes.iter().sum();
        let sb: i32 = bob_sizes.iter().sum();
        for x in alice_sizes.into_iter() {
            if bob_sizes.contains(&(x + (sb - sa) / 2)) {
                return vec![x, x + (sb - sa) / 2];
            }
        }
        vec![]
    }
}
