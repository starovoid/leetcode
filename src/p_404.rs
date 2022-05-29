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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = 0;
        sum(root.as_ref(), &mut s);
        s
    }
}

fn sum(node_opt: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32){
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        let left_opt = node.left.as_ref();
        if left_opt.is_some() {
            let left = (*left_opt.unwrap()).borrow();
            if left.left.is_none() && left.right.is_none() {
                *s += left.val;
            } else {
                sum(left_opt, s);
            }
        }
        sum(node.right.as_ref(), s);
    }
}
