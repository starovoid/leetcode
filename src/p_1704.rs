impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        s.chars().take(s.len() / 2).filter(|c| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c)).count()
            == s.chars().skip(s.len() / 2).filter(|c| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c)).count()
    }
}
