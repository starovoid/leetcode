impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        let mut res = Vec::new();
        for i in (0..prices.len()).rev(){ 
            while stack.len() > 0 && prices[i] < stack[stack.len() - 1]{
                stack.pop();
            }
            
            match stack.last(){
                Some(x) => res.push(prices[i] - x),
                None => res.push(prices[i]),
            }
            stack.push(prices[i]);
        }
        res.reverse();
        res
    }
}
