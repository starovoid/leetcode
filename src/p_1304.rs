impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = (1..=n/2).chain((1..=n/2).map(|x| -x)).collect();
        if n % 2 == 1 {
            ans.push(0);
        }
        ans
    }
}
