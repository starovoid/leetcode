const PRIMES: [i32; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let p = PRIMES.iter().take_while(|&&x| x <= n).count() as u128;
        ((factorial(n as u128 - p) * factorial(p)) % 1000000007) as i32
    }
}

fn factorial(n: u128) -> u128 {
    let mut fact = 1;
    for i in 2..=n {
        fact = (fact * i) % 1000000007;
    }
    fact
}
