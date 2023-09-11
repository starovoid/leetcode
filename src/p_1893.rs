impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut covered = vec![false; 51];
        for r in ranges.iter() {
            for i in r[0]..=r[1] {
                covered[i as usize] = true;
            }
        }
        (left..=right).all(|i| covered[i as usize])
    }
}
