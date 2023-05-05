impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;
        while i < arr.len() {
            let old_i = i;
            for p in pieces.iter() {
                let mut j = 0;
                while j < p.len() && i < arr.len() && arr[i] == p[j] {
                    j += 1;
                    i += 1;
                }
            }
            if i == old_i {
                return false;
            }
        }
        true
    }
}
