impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        (helper(&date1) - helper(&date2)).abs()
    }
}

fn helper(date: &str) -> i32 {
    let p = date.split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut y = p[0];
    let mut m = p[1];
    let d = p[2];
    if m < 3 {
        m += 12;
        y -= 1;
    }
    365 * y + y / 4 + y / 400 - y / 100 + d + (153 * m + 8) / 5
}
