impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut even = nums.iter().filter(|&x| x % 2 == 0);
        let mut odd = nums.iter().filter(|&x| x % 2 == 1);
        let mut ans = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            if i % 2 == 0 {
                ans.push(*even.next().unwrap());
            } else {
                ans.push(*odd.next().unwrap());
            }
        }
        ans
    }
}
