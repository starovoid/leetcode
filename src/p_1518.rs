impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total = 0;
        let mut rem = 0;
        while num_bottles > 0 {
            total += num_bottles;
            let r = (num_bottles + rem) % num_exchange;
            num_bottles = (num_bottles + rem) / num_exchange;
            rem = r;
        }
        total
    }
}
