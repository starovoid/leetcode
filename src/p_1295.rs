impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|n| {
            let mut x = *n;
            let mut cnt = 0;
            while x > 0 {
                cnt += 1;
                x /= 10;
            }
            cnt % 2 == 0
        })
        .count() as i32
    }
}
