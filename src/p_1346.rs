use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(arr.len());
        set.extend(arr.iter().copied());
        let zcnt = arr.iter().filter(|&x| *x == 0).count();
        for x in arr {
            if x != 0 && set.contains(&(x * 2)) || x == 0 && zcnt > 1 {
                return true;
            }
        }
        false
    }
}
