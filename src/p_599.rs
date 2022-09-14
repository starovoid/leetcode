use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut idx2: HashMap<String, usize> = list2.into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();
        
        let mut min_sum = list1.len() + idx2.len();
        let mut ans: Vec<String> = Vec::new();
        for (i, s) in list1.into_iter().enumerate() {
            match idx2.get(&s) {
                Some(j) => {
                    if i + j < min_sum {
                        min_sum = i + j;
                        ans = vec![s];
                    } else if i + j == min_sum {
                        ans.push(s);
                    }
                },
                None => {},
            }
        }
        
        ans
    }
}
