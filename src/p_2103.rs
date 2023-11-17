impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = [0; 10];
        let mut r = rings.as_bytes();
        for i in (0..r.len()-1).step_by(2) {
            let p = (r[i + 1] as u8 - b'0') as usize;
            match r[i] {
                b'R' => rods[p] |= 0b100,
                b'G' => rods[p] |= 0b010,
                b'B' => rods[p] |= 0b001,
                _ => {},
            }
        }
        rods.iter().filter(|&&x| x == 7).count() as i32
    }
}
