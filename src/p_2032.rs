impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counter = [0u8; 101];
        nums1.iter().for_each(|&x| counter[x as usize] |= 0b001);
        nums2.iter().for_each(|&x| counter[x as usize] |= 0b010);
        nums3.iter().for_each(|&x| counter[x as usize] |= 0b100);
        counter.into_iter().enumerate().filter(|(i, &cnt)| cnt.count_ones() >= 2).map(|(i, _)| i as i32).collect()
    }
}
