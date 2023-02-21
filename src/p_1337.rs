use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Ones, idx
#[derive(Eq, PartialEq)]
struct Row(i32, i32);

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0).then(other.1.cmp(&self.1))
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.cmp(&self.0).then(other.1.cmp(&self.1)))
    }
}


impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        heap.extend(mat.iter().enumerate().map(|(i, row)| {
            let mut ones = 0;
            for &x in row.iter() {
                if x == 1 {
                    ones += 1;
                } else {
                    break;
                }
            }
            Row(ones, i as i32)
        }));
        
        let mut ans = Vec::with_capacity(k as usize);
        for i in 0..(k as usize) {
            ans.push(heap.pop().unwrap().1);
        }

        ans
    }
}
