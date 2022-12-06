truct RecentCounter {
    requests: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            requests: Vec::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        let left = t - 3000;
        let right = t;
        self.requests.push(t);
        self.requests.iter().filter(|&x| *x >= left && *x <= right).count() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
