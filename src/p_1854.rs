impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut alive = [0i32; 101];
        for log in logs.iter() {
            alive[log[0] as usize - 1950] += 1;
        }
        for i in 1..101 {
            alive[i] += alive[i-1];
        }

        let mut dead = [0i32; 101];
        for log in logs.iter() {
            dead[log[1] as usize - 1950] += 1;
        }
        for i in 1..101 {
            dead[i] += dead[i-1];
        }

        (0..101).rev().max_by_key(|&i| alive[i as usize] - dead[i as usize]).unwrap() + 1950
    }
}
