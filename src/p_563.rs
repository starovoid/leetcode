// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum_tilt = 0;
        work(root.as_ref(), &mut sum_tilt);
        sum_tilt
    }
}

fn work(node_opt: Option<&Rc<RefCell<TreeNode>>>, sum_tilt: &mut i32) -> i32 {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        let left_sum = work(node.left.as_ref(), sum_tilt);
        let right_sum = work(node.right.as_ref(), sum_tilt);
        *sum_tilt += (left_sum - right_sum).abs();
        left_sum + right_sum + node.val
    } else {
        0
    }
}
