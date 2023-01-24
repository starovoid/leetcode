static WEEKDAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };

        let days = (1971..year)
            .map(|y| 365 + is_leap(y) as i32)
            .sum::<i32>()
            + (0..month - 1)
                .map(|m| months[m as usize] + (m == 1 && is_leap(year)) as i32)
                .sum::<i32>()
            + day;

        let res_day = (4 + (days - 1)) % 7;
        WEEKDAYS[res_day as usize].to_owned()
    }
}
