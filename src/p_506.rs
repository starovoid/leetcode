impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();

        let mut score_sort = score.clone();
        score_sort.sort_unstable();

        let m1 = score_sort.pop().unwrap_or(-1);
        let m2 = score_sort.pop().unwrap_or(-1);
        let m3 = score_sort.pop().unwrap_or(-1);

        score
            .iter()
            .map(|&x| match x {
                _ if x == m1 => "Gold Medal".to_string(),
                _ if x == m2 => "Silver Medal".to_string(),
                _ if x == m3 => "Bronze Medal".to_string(),
                _ => score_sort
                    .binary_search(&x)
                    .map_or(0, |i| n - i)
                    .to_string(),
            })
            .collect()
    }
}
