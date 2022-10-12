use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let start_color = image[sr as usize][sc as usize];
        if start_color == color {
            return image;
        }
        image[sr as usize][sc as usize] = color;
        
        let mut deque: VecDeque<(usize, usize)> = VecDeque::from([(sr as usize, sc as usize)]);
        
        while let Some((row, col)) = deque.pop_front() {
            if row > 0 && image[row-1][col] == start_color {
                image[row-1][col] = color;
                deque.push_back((row-1, col));
            }
            if row < image.len() - 1 && image[row+1][col] == start_color {
                image[row+1][col] = color;
                deque.push_back((row+1, col));
            }
            if col > 0 && image[row][col-1] == start_color {
                image[row][col-1] = color;
                deque.push_back((row, col-1));
            }
            if col < image[0].len() - 1 && image[row][col+1] == start_color {
                image[row][col+1] = color;
                deque.push_back((row, col+1));
            }
        }
        
        image
    }
}
