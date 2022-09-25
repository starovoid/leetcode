impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r = img.len();
        let c = img[0].len();
        let mut res:Vec<Vec<i32>> = vec![vec![0;c];r];
        for i in 0..r {
            for j in 0..c {
                let mut sum = img[i][j];
                let mut cnt = 1;
                if i > 0 && j > 0 {
                    sum += img[i-1][j-1];
                    cnt += 1;
                }
                if i > 0  {
                    sum += img[i-1][j];
                    cnt += 1;
                }
                if i>0 && j+1 < c {
                    sum += img[i-1][j+1];
                    cnt += 1;
                }
                if j > 0 {
                    sum += img[i][j-1];
                    cnt += 1;
                }
                if j + 1 < c {
                    sum += img[i][j+1];
                    cnt += 1;
                }
                if i+1 < r && j > 0 {
                    sum += img[i+1][j-1];
                    cnt += 1;
                }
                if i+1 < r {
                    sum += img[i+1][j];
                    cnt += 1;
                }
                if i+1 < r && j+1 < c {
                    sum += img[i+1][j+1];
                    cnt += 1;
                }
                res[i][j] = sum / cnt;
            }
         }
        res
    }
}
