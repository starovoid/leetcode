impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let idx = |c| (c as u8 - b'a') as usize;
        let mut duration = vec![0; 26];
        duration[idx(keys_pressed.chars().next().unwrap())] = release_times[0];
        for (i, c) in keys_pressed.chars().enumerate().skip(1) {
            duration[idx(c)] = duration[idx(c)].max(release_times[i] - release_times[i - 1]);
        }
        (0..26).max_by(|&i, j| duration[i].cmp(&duration[*j]).then(i.cmp(j))).map(|i| (i as u8 + b'a') as char).unwrap()
    }
}
