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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let (ld, lp) = depth(root.as_ref(), x, 1, -1);
        let (rd, rp) = depth(root.as_ref(), y, 1, -1);
        lp != rp && ld == rd
    }
}

fn depth(node_opt: Option<&Rc<RefCell<TreeNode>>>, target: i32, d: i32, parent: i32) -> (i32, i32) {
    if let Some(node_ref) = node_opt {
        let node = node_ref.borrow();
        if node.val == target {
            return (d, parent);
        }
        let (left_d, left_p) = depth(node.left.as_ref(), target, d + 1, node.val);
        if left_d > 0 {
            return (left_d, left_p);
        }
        let (right_d, right_p) = depth(node.right.as_ref(), target, d + 1, node.val);
        if right_d > 0 {
            return (right_d, right_p);
        }
        (-1, parent)
    } else {
        (-1, parent)
    }
}
