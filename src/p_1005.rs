impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut neg: Vec<i32> = nums.iter().filter(|&x| *x < 0).cloned().collect();
        let mut sum = nums.iter().filter(|&x| *x > 0).sum::<i32>();
        if k as usize > neg.len() {
            sum += -neg.iter().sum::<i32>();
            let min_pos = nums.into_iter().filter(|x| *x >= 0).min();
            let max_neg = neg.iter().max();
            match (min_pos, max_neg) {
                (Some(mp), Some(mn)) => {
                    let x = (-1i32).pow((k as usize - neg.len()) as u32) * mp;
                    let y = (-1i32).pow((k as usize - neg.len() + 1) as u32) * mn;
                    if x >= y {
                        sum -= mp;
                        sum += x;
                    } else {
                        sum += mn;
                        sum += y;
                    }
                },
                (Some(mp), None) => {
                    sum -= mp;
                    sum += (-1i32).pow((k as usize - neg.len()) as u32) * mp;
                },
                (None, Some(mn)) => {
                    sum += mn;
                    sum += (-1i32).pow((k as usize - neg.len() + 1) as u32) * mn;
                },
                _ => {},
            };
        } else {
            neg.sort();
            sum += -neg.iter().take(k as usize).sum::<i32>();
            if neg.len() > k as usize{
                sum += neg.iter().rev().take(neg.len() - k as usize).sum::<i32>();
            }
        }
        sum
    }
}
