impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let mut res = Vec::with_capacity((s.len() as f64 / k as f64).ceil() as usize);
        for i in (0..s.len()).step_by(k as usize) {
            res.push(s[i..(i + k).min(s.len())].to_string());
        }
        if s.len() % k != 0 {
            res.last_mut().map(|part| part.extend(std::iter::repeat(fill).take(k - s.len() % k)));
        }
        res
    }
}
