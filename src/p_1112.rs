impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut counter = [0; 1001];
        let mut res = arr1;
        res.iter().for_each(|&i| counter[i as usize] += 1);

        let mut pos = 0;
        for &i in arr2.iter() {
            let num = &mut counter[i as usize];
            for _ in 0..*num {
                res[pos] = i;
                pos += 1;
            }
            *num = 0;
        }

        for (val, &n) in counter.iter().enumerate().filter(|(_, &n)| n > 0) {
            for _ in 0..n {
                res[pos] = val as i32;
                pos += 1;
            }
        }
        res
    }
}
