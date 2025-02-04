use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn sort_even_odd(mut nums: Vec<i32>) -> Vec<i32> {
        let mut even: BinaryHeap<Reverse<i32>> = nums.iter().step_by(2).map(|&x| Reverse(x)).collect();
        let mut odd: BinaryHeap<i32> = nums.iter().copied().skip(1).step_by(2).collect();

        let mut temp_item = false;
        if odd.len() < even.len() {
            odd.push(i32::MIN);
            temp_item = true;
        }

        nums.clear();
        while let (Some(Reverse(a)), Some(b)) = (even.pop(), odd.pop()) {
            nums.push(a);
            nums.push(b);
        }

        if temp_item {
            nums.pop();
        }

        nums

    }
}
