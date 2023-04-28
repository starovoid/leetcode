impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        for x in 0..=nums.len() {
            let mut i = match nums.binary_search(&(x as i32)) {
                Ok(n) => n,
                Err(n) => n,
            };

            let mut prev = 0;
            let mut j = i;
            while j > 0 && nums[j - 1] == x as i32 {
                prev += 1;
                j -= 1;
            }
            
            if nums.len() - i + prev == x {
                return x as i32;
            }
        }

        -1
    }
}
