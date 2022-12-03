impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if typed.len() < name.len() {
            return false;
        }
        let name: Vec<char> = name.chars().collect();
        let typed: Vec<char> = typed.chars().collect();
        let mut i = 0;
        let mut j = 0;
        let mut cnt_name = 0;
        let mut cnt_typed = 0;
        while i < name.len() && j < typed.len() {
            cnt_name = 0;
            cnt_typed = 0;
            let target = name[i];
            while i < name.len() && name[i] == target {
                i += 1;
                cnt_name += 1;
            }
            while j < typed.len() && typed[j] == target {
                j += 1;
                cnt_typed += 1;
            }
            if cnt_typed < cnt_name {
                return false;
            }
        }
        j == typed.len() && i == name.len()
    }
}
