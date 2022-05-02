impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        //nums.sort_by_key(|&x| x == 0)
        
        /*let mut read = 0;
        let mut write = 0;
        
        while read < nums.len() {
            if nums[read] != 0 {
                nums[write] = nums[read];
                write += 1;
            }
            read += 1;
        }
        
        while write < nums.len() {
            nums[write]= 0;
            write += 1;
        }*/
        
        let mut n = nums.len();
        nums.retain(|&x| x != 0);
        nums.extend(std::iter::repeat(0).take(n - nums.len()));
    }
}
