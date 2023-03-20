use std::collections::HashMap;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target_cnt = HashMap::new();
        let mut arr_cnt = HashMap::new();
        for x in target {
            *target_cnt.entry(x).or_insert(0) += 1;
        }
        for x in arr {
            *arr_cnt.entry(x).or_insert(0) += 1;
        }
        arr_cnt == target_cnt
    }
}
