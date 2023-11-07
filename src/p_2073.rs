impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        tickets[0..=k].iter().map(|&t| t.min(tickets[k])).sum::<i32>() 
            + tickets[k+1..].iter().map(|&t| t.min(tickets[k] - 1)).sum::<i32>()
    }
}
