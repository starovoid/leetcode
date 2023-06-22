impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 101];
        for x in nums {
            counter[x as usize] += 1;
        }
        counter.into_iter().enumerate().filter(|(x, cnt)| *cnt == 1).map(|(i, _)| i as i32).sum::<i32>()
    }
}
