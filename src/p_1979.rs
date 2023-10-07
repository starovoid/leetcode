impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while a != 0 && b != 0 {
                if a > b {
                    a = a % b;
                } else {
                    b = b % a;
                }
            }
            a + b
        }

        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();

        gcd(min, max)
    }
}
