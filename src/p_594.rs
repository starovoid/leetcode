impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        
        let mut map = std::collections::HashMap::new();
        for n in nums.into_iter() {
            *map.entry(n).or_insert(0) += 1;
        }
        
        for (x, c) in map.iter() {
            if let Some(cnt) = map.get(&(x+1)) {
                ans = ans.max(c + cnt)
            }
        }
        
        ans
    }
}
