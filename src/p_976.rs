impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();
        for i in 0..nums.len()-2 {
            if nums[i+1] + nums[i+2] > nums[i] {
                return nums[i] + nums[i+1] + nums[i+2];
            }
        }
        0
    }
}
