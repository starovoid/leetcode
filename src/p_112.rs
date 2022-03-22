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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        has_path(target_sum, root.as_ref())
    }
}

fn has_path(target_sum: i32, node_opt: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match node_opt {
        None => false,
        Some(node_ref) => {
            let node = (*node_ref).borrow();
            let val = node.val;
            match (node.left.as_ref(), node.right.as_ref()) {
                (None, None) => val == target_sum,
                (left, right) => has_path(target_sum - val, left) 
                    || has_path(target_sum - val, right)
            }
        }
    }
}
