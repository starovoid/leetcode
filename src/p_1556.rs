impl Solution {
    pub fn thousand_separator(n: i32) -> String {
       n.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(".")
    }
}
