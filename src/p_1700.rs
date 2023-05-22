use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut students = VecDeque::from(students);
        sandwiches.reverse();

        let mut n = 0;
        while sandwiches.len() > 0 && sandwiches.len() > n {
            let stud = students.pop_front().unwrap();
            let sand = sandwiches.pop().unwrap();
            if stud == sand{
                n = 0;
            } else {
                sandwiches.push(sand);
                students.push_back(stud);
                n += 1;
            }
        }
        
        students.len() as i32
    }
}
