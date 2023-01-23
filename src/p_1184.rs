use std::mem;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, mut start: i32, mut dest: i32) -> i32 {
        if start > dest {
            mem::swap(&mut start, &mut dest);
        }
        let s1: i32 = distance[(start as usize)..(dest as usize)].iter().sum();
        let s2: i32 = distance[0..(start as usize)].iter().sum::<i32>() + distance[(dest as usize)..distance.len()].iter().sum::<i32>();
        s1.min(s2)
    }
}
