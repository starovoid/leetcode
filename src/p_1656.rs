struct OrderedStream {
    data: Vec<String>,
    idx: usize,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        OrderedStream {
            data: vec!["".to_owned(); n as usize + 1],
            idx: 1,
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.data[id_key as usize] = value;
        let mut res = Vec::new();
        if id_key as usize == self.idx {
            while self.idx < self.data.len() && !self.data[self.idx].is_empty() {
                res.push(self.data[self.idx].clone());
                self.idx += 1;
            }
        }
        res
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */
