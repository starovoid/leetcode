impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut count = 0;
        for w in text.split_ascii_whitespace() {
            count += 1;
            for c in w.chars() {
                if broken_letters.contains(c) {
                    count -= 1;
                    break;
                }
            }
        }
        count
    }
}
