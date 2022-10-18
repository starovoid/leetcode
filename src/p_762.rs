impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19];
        (left..=right).filter(|x| primes.contains(&x.count_ones())).count() as i32
    }
}
