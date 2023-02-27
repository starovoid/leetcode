impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0i32; 101];
        for &x in nums.iter() {
            count[x as usize] += 1;
        }

        let mut ans = vec![0i32; 101];
        for i in 1..=100 {
            ans[i] = ans[i - 1] + count[i - 1];
        }

        nums.into_iter().map(|x| ans[x as usize]).collect()
    }
}
