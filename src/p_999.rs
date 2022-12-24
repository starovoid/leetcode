impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        fn calc_cap(it: impl Iterator<Item = char>, rook_pos: usize) -> i32 {
            let (mut res, mut num) = (0, 0);
            for (pos, fig) in it.enumerate() {
                match (fig, pos < rook_pos) {
                    ('R', _) => res += num,
                    ('p', true) => num = 1,
                    ('B', true) => num = 0,
                    ('p', false) => return res + 1,
                    ('B', false) => return res,
                    _ => (),
                }
            }
            res
        }

        for (y, row) in board.iter().enumerate() {
            if let Some(x) = row.iter().position(|c| *c == 'R') {
                let row_it = row.iter().cloned();
                let col_it = (0..8_usize).map(|i| board[i][x]);
                return calc_cap(row_it, x) + calc_cap(col_it, y);
            }
        }

        -1
    }
}
