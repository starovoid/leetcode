impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = vec![0usize; nums.len()];
        for x in nums.into_iter() {
            counter[x as usize - 1] += 1;
        }
        let mut ans = vec![-1, -1];
        for (i, cnt) in counter.into_iter().enumerate() {
            if cnt == 0 {
                ans[1] = i as i32 + 1;
            } else if cnt == 2 {
                ans[0] = i as i32 + 1;
            }
        }
        ans
    }
}
