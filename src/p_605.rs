impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut vacant = 0;
        
        let mut i = 0;
        while i < flowerbed.len() {
            if flowerbed[i] == 0 {
                let mut j = i + 1;
                while j < flowerbed.len() && flowerbed[j] == 0 {
                    j += 1;
                }
                
                let mut cnt = j - i;
                if (i == 0 && flowerbed[i] == 0) && (j == flowerbed.len() && flowerbed[j-1] == 0) {
                    cnt += 2;
                }
                else if (i == 0 && flowerbed[i] == 0) || (j == flowerbed.len() && flowerbed[j-1] == 0) {
                    cnt += 1;
                }
                vacant += (cnt - 1) / 2;
                
                i = j;
            } else {
                i += 1;
            }
        }
        
        vacant as i32 >= n
    }
}
