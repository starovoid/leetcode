impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut m1 = cost[0];
        let mut m2 = cost[1];
        for (i, x) in cost.into_iter().enumerate().skip(2) {
            let m3 = m1.min(m2) + x;
            m1 = m2;
            m2 = m3;
        }
        m1.min(m2)
    }
}
