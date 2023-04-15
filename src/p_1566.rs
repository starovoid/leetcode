impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;

        for start in 0..arr.len() {
            if m * k > arr.len() - start {
                return false;
            }

            let mut count = 0;
            let mut j = 0;
            while j < arr.len() {
                if j + m <= arr.len() && arr[j..j+m] == arr[start..start+m] {
                    count += 1;
                    j += m;
                } else {
                    j += 1;
                    if count >= k {
                        return true;
                    }
                    count = 0;
                }
            }

            if count >= k {
                return true;
            }
        }

        false
    }
}
