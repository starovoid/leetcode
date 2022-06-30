impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num != 1 && num == 1 + (2..(num as f64).sqrt().ceil() as i32)
            .map(|d| if num % d == 0 { d + num/d } else { 0 })
            .sum::<i32>()
    }
}
