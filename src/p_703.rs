use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap: BinaryHeap<Reverse<i32>> = nums.into_iter()
            .map(|x| Reverse(x))
            .collect();
        while heap.len() > k as usize {
            heap.pop();
        }
        KthLargest {
            k: k as usize,
            heap: heap,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        match self.heap.peek() {
            Some(&Reverse(kth_min)) => {
                if val > kth_min || self.heap.len() < self.k {
                    self.heap.push(Reverse(val));
                    if self.heap.len() > self.k {
                        self.heap.pop();
                    }
                }
            }
            None => self.heap.push(Reverse(val)),
        }
        self.heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
