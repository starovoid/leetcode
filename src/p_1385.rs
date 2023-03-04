impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut dist = 0;
        for x in arr1.iter() {
            if arr2.iter().all(|y| (x - y).abs() > d) {
                dist += 1;
            }
        }
        dist
    }
}
