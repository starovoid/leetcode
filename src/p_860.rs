impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = [0, 0];
        for b in bills.into_iter() {
            match b {
                5 => change[0] += 1,
                10 => {
                    if change[0] < 1 {
                        return false;
                    }
                    change[1] += 1;
                    change[0] -= 1;
                },
                20 => {
                    if change[1] >= 1 && change[0] >= 1 {
                        change[1] -= 1;
                        change[0] -= 1;
                    } else if change[0] >= 3 {
                        change[0] -= 3;
                    } else {
                        return false;
                    }
                },
                _ => {},
            }
        }
        true
    }
}
Console
