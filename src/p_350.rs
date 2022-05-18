use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1: HashMap<i32, usize> = HashMap::new();
        for &x in nums1.iter() {
            *map1.entry(x).or_insert(0) += 1;
        }
        let mut map2: HashMap<i32, usize> = HashMap::new();
        for &x in nums2.iter() {
            *map2.entry(x).or_insert(0) += 1;
        }
        
        let mut ans: Vec<i32> = Vec::new();
        for (x, cnt1) in map1.iter() {
            if let Some(&cnt2) = map2.get(x) {
                ans.append(&mut vec![*x; cnt2.min(*cnt1)]);
            }
        }
        ans
    }
}
