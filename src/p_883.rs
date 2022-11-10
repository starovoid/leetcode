impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
	    grid.iter().map(|p| p.iter().filter(|z| **z > 0).count() as i32).sum::<i32>()
            +
	    grid.iter().map(|p| *p.iter().max().unwrap()).sum::<i32>()
            +
	    (0..grid.len()).map(|i| grid.iter().map(|p| p[i]).max().unwrap()).sum::<i32>()
    }
}
