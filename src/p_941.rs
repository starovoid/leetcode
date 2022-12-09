impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut i = 0;
        while i < arr.len()-1 && arr[i] < arr[i+1] {
            i += 1;
        }
        if i == 0 || i == arr.len() - 1 {
            return false;
        }
        for j in i..arr.len()-1 {
            if arr[j+1] >= arr[j] {
                return false;
            }
        }
        true
    }
}
