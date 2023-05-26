impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|x| -x[1]);
        let mut count = 0;
        let mut free = truck_size;
        for x in box_types {
            let num = x[0].min(free);
            free -= num;
            count += num * x[1];
            if free <= 0 {
                break;
            }
        }
        count
    }
}
