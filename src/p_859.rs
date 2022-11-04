impl Solution{
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let diff = a
            .chars()
            .zip(b.chars())
            .filter(|(c, d)| c != d)
            .collect::<Vec<(char, char)>>();
        if diff.len() == 0 {
            let mut set = std::collections::HashSet::new();
            for i in 0..a.len() {
                if set.contains(&a[i..(i + 1)]) {
                    return true;
                }
                set.insert(&a[i..(i + 1)]);
            }
            false
        } else if diff.len() == 2 {
            let diff_a = diff[0];
            let diff_b = diff[1];
            diff_a.0 == diff_b.1 && diff_a.1 == diff_b.0 && diff_b.0 != diff_a.0
        } else {
            false
        }
    }
}
