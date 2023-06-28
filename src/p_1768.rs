impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();
        let mut s = String::with_capacity(word1.len() + word2.len());
        
        loop {
            if let Some(c1) = chars1.next() {
                s.push(c1);
            } else {
                break;
            }
            if let Some(c2) = chars2.next() {
                s.push(c2);
            } else {
                break;
            }
        }

        s.extend(chars1);
        s.extend(chars2);

        s
    }
}
