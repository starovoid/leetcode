impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;
        
        for p in prices {
            profit = profit.max(p - buy);
            buy = buy.min(p);
        }
        
        profit
    }
}
