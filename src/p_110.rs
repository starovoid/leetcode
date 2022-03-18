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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check(root.as_ref()).1
    }
}

fn check(node_opt: Option<&Rc<RefCell<TreeNode>>>) -> (i64, bool) {
    if let Some(node) = node_opt {
        let dl = check((*node.borrow()).left.as_ref());
        let dr = check((*node.borrow()).right.as_ref());
        (dl.0.max(dr.0) + 1, dl.1 && dr.1 && (dl.0 - dr.0).abs() <= 1)
    } else {
        (0, true)
    }
}
