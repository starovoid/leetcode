struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue{
            input: vec![],
            output: vec![],
        }
    }
    
    fn push(&mut self, x: i32) {
        self.input.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.peek();
        self.output.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.output.is_empty() {
            while self.input.len() > 0 {
                self.output.push(self.input.pop().unwrap());
            }

        }
        self.output.last().cloned().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}
