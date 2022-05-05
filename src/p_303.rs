struct NumArray {
    sums: Vec<i32>,
}


impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = Vec::with_capacity(nums.len() + 1);
        sums.push(0);
        
        for (_, x) in nums.into_iter().enumerate() {
            sums.push(*sums.last().unwrap() + x);
        }
        
        NumArray {
            sums: sums,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize + 1] - self.sums[left as usize]
    }
}
