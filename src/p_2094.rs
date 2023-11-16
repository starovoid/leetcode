use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(mut digits: Vec<i32>) -> Vec<i32> {
        let mut nums = HashSet::new();
        for i in 0..digits.len()-2 {
            for j in i+1..digits.len()-1 {
                for k in j+1..digits.len() {
                    let n1 = digits[k]*100 + digits[j]*10 + digits[i];
                    let n2 = digits[k]*100 + digits[i]*10 + digits[j];
                    let n3 = digits[j]*100 + digits[k]*10 + digits[i];
                    let n4 = digits[j]*100 + digits[i]*10 + digits[k];
                    let n5 = digits[i]*100 + digits[j]*10 + digits[k];
                    let n6 = digits[i]*100 + digits[k]*10 + digits[j];
                    nums.extend([n1, n2, n3, n4, n5, n6].into_iter().filter(|&&x| x % 2 == 0 && x > 99));
                }
            }
        }
        let mut res: Vec<i32> = nums.into_iter().collect();
        res.sort_unstable();
        res
    }
}
