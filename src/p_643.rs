impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut s = nums.iter().take(k).map(|x| *x).sum::<i32>();
        let mut max_s = s;
        for i in 0..(nums.len()-k) {
            s -= nums[i];
            s += nums[i+k];
            max_s = max_s.max(s);
        }
        (max_s as f64) / (k as f64)
    }
}
