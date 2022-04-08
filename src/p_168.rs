impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number as u32;
        let mut num = String::new();
        
        while column_number > 0 {
            if column_number % 26 == 0 {
                num.push('Z');
                column_number /= 27;
            } else {
                num.push(char::from_digit((column_number % 27) + 9, 36).unwrap());
                column_number /= 2u;
            }
        }
        
        num.to_uppercase().chars().rev().collect()
    }
}
