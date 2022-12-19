use std::mem::swap;

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut k: Vec<i32> = k.to_string().chars().map(|c| (c as u8 - b'0') as i32).collect();
        if num.len() < k.len() {
            swap(&mut num, &mut k);
        }

        let nl = num.len();
        let mut c = 0;
        for i in (0..k.len()).rev() {
            let t = num[nl - k.len() + i] + k[i] + c;
            c = t / 10;
            num[nl - k.len() + i] = t % 10;
        }

        println!("{} | {:?}", c, num);
        if nl != k.len() {
            let mut i = num.len() - k.len() - 1;
            while c == 1 && i >= 0 && i < num.len( ){
                let t = num[i] + c;
                c = t / 10;
                num[i] = t % 10;
                i -= 1;
            }
        }

        if c == 1 {
            num.insert(0, 1);
        }

        num
    }
}
