impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let f = first_word.chars().map(|c| (c as u8 - b'a' + b'0') as char).collect::<String>().parse::<u32>().unwrap();
        let s = second_word.chars().map(|c| (c as u8 - b'a' + b'0') as char).collect::<String>().parse::<u32>().unwrap();
        let t = target_word.chars().map(|c| (c as u8 - b'a' + b'0') as char).collect::<String>().parse::<u32>().unwrap();
        f + s == t
    }
}
