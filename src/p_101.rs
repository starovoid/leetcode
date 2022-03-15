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
use std::cell::{RefCell, Ref};
use std::collections::VecDeque;
impl<'a> Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match_subtrees(
            (*root.as_ref().unwrap()).borrow().left.as_ref(),
            (*root.as_ref().unwrap()).borrow().right.as_ref(),
        )
    }
}

fn match_subtrees(
    left: Option<&Rc<RefCell<TreeNode>>>, 
    right: Option<&Rc<RefCell<TreeNode>>>,
) -> bool 
{
    if left.is_some() && right.is_some() {
        (**left.unwrap()).borrow().val == (**right.unwrap()).borrow().val 
        &&
        match_subtrees(
            (**left.unwrap()).borrow().left.as_ref(),
            (**right.unwrap()).borrow().right.as_ref(),
        )
        &&
        match_subtrees(
            (**left.unwrap()).borrow().right.as_ref(),
            (**right.unwrap()).borrow().left.as_ref(),
        )
    }
    else {
        left.is_some() == right.is_some()
    }
}
