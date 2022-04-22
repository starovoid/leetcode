impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return Vec::new();
        }
        
        let mut ranges: Vec<String> = Vec::new();
        let mut a = nums[0];
        let mut b = nums[0];
        
        for &x in nums.iter().skip(1) {
            if b + 1 < x {
                if a == b {
                    ranges.push(b.to_string());
                } else {
                    ranges.push(format!("{}->{}", a, b));
                }
                a = x;
            }
            b = x;
        }
        
        if a == b {
            ranges.push(b.to_string());
        } else {
            ranges.push(format!("{}->{}", a, b));
        }
        
        ranges
    }
}
