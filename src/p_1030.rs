use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r: i32, c: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity((rows * cols) as usize);
        let mut used: HashSet<(i32, i32)> = HashSet::with_capacity((rows * cols) as usize);
        let mut deque: VecDeque<(i32, i32)> = VecDeque::new();
        deque.push_back((r, c));
        used.insert((r, c));

        while let Some((row, col)) = deque.pop_front() {
            ans.push(vec![row, col]);
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if 0 <= row + dr && row + dr < rows && 0 <= col + dc && col + dc < cols && used.insert((row + dr, col + dc)) {
                    deque.push_back((row + dr, col + dc));
                }
            }
        }

        ans
    }
}
