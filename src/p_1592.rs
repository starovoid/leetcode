impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let count_whitespace = text.chars().filter(|x| *x == ' ').count();
        if count_whitespace == 0 {
            return text;
        }
        let div_mod = |x, y| (x / y, x % y);
        let words = text.split_whitespace().collect::<Vec<_>>();

        if words.len() == 1 {
            words[0].to_owned() + " ".repeat(count_whitespace).as_str()
        } else {
            let (div_sep, mod_sep) = div_mod(count_whitespace, words.len() - 1);
            words.join(" ".repeat(div_sep).as_str()) + " ".repeat(mod_sep).as_str()
        }
    }
}
