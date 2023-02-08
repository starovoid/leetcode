impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![vec![' '; 3]; 3];
        for (i, m) in moves.iter().enumerate() {
            if i % 2 == 0 {
                board[m[0] as usize][m[1] as usize] = 'X';
            } else {
                board[m[0] as usize][m[1] as usize] = 'O';
            }
        }

        if board[1][1] != ' ' && (board[0][0] == board[1][1] && board[1][1] == board[2][2]
            || board[0][2] == board[1][1] && board[1][1] == board[2][0]) {
                return (if board[1][1] == 'X' { 'A' } else { 'B' }).to_string();
        }

        for i in 0..3 {
            if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
                return (if board[0][i] == 'X' { 'A' } else { 'B' }).to_string();
            }
            if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] { 
                return (if board[i][0] == 'X' { 'A' } else { 'B' }).to_string();
            }
        }

        if moves.len() == 9 {
            "Draw".to_owned()
        } else {
            "Pending".to_owned()
        }
    }
}
