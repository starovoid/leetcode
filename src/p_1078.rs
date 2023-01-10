impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words = text.split(" ").collect::<Vec<&str>>();
        let mut ans = Vec::new();
        for i in 0..words.len()-2 {
            if words[i] == first && words[i+1] == second {
                ans.push(words[i+2].to_owned());
            }
        }
        ans
    }
}
