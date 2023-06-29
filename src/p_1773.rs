impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        items.into_iter().filter(|item| rule_value == match rule_key.as_ref() {
            "type" => &item[0],
            "color" => &item[1],
            "name" => &item[2],
            _ => "",
        })
        .count() as i32
    }
}
