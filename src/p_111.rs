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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        depth(root.as_ref())
    }
}

fn depth(node_opt: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = node_opt {
        let left_depth = depth((*node).borrow().left.as_ref());
        let right_depth = depth((*node).borrow().right.as_ref());
        
        if left_depth == 0 && right_depth == 0 {
            1
        } else if left_depth == 0 {
            right_depth + 1
        } else if right_depth == 0 {
            left_depth + 1
        } else {
            left_depth.min(right_depth) + 1
        }
        
    } else {
        0
    }
}
