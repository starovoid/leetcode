use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut mem: HashMap<i32, (usize, usize, usize)> = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            let entry = mem.entry(x).or_insert((i, i, 0));
            entry.2 += 1;
            entry.1 = i;
        }
        mem.into_iter()
            .map(|(_, v)| v)
            .max_by(|x, y| x.2.cmp(&y.2).then((y.1-y.0).cmp(&(x.1-x.0))))
            .map(|(left, right, _)| right - left)
            .unwrap() as i32 + 1
    }
}
