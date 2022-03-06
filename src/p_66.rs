impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = true;
        let mut i = digits.len();
        
        while carry && i > 0 {
            i -= 1;
            carry = digits[i] == 9;
            digits[i] = (digits[i] + 1) % 10;
        }
        
        if i == 0 && carry {
            digits.insert(0, 1);
        }
        
        digits
    }
}
