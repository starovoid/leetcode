struct ParkingSystem {
    max_big: i32,
    max_medium: i32,
    max_small: i32,
    big: i32,
    medium: i32,
    small: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { 
            max_big: big, 
            max_medium: medium, 
            max_small: small,
            big: 0,
            medium: 0,
            small: 0, 
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => if self.big == self.max_big { return false } else { self.big += 1},
            2 => if self.medium == self.max_medium { return false } else { self.medium += 1},
            3 => if self.small == self.max_small { return false } else { self.small += 1},
            _ => return false,
        }
        true
    }
}
