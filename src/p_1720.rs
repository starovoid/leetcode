impl Solution {
    pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        let mut ans = vec![first];
        for x in encoded.into_iter() {
            first ^= x;
            ans.push(first);
        }
        ans
    }
}
