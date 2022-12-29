impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let s = arr.iter().sum::<i32>();
        if s % 3 != 0 {
            return false;
        }

        let mut i = 0;
        let mut left_sum = arr[i];
        while i < arr.len() - 2 && left_sum != s/3 {
            i += 1;
            left_sum += arr[i];
        }
        if i >= arr.len() - 1 {
            return false;
        }

        let mut j = i + 1;
        let mut right_sum = arr[j];
        while j < arr.len() - 1 && right_sum != s/3 {
            j += 1;
            right_sum += arr[j];
        }
        j < arr.len() - 1
    }
}
