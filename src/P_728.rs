impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for num in left..=right {
            let mut x = num;
            let mut sd = true;
            while x > 0 {
                if x % 10 == 0 || num % (x % 10) != 0 {
                    sd = false;
                    break;
                }
                x /= 10;
            }
            if sd {
                ans.push(num);
            }
        }
        ans
    }
}
