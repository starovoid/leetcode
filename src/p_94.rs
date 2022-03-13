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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        traversal(&mut res, root.as_ref());
        res
    }
}

fn traversal(res: &mut Vec<i32>, root: Option<&Rc<RefCell<TreeNode>>>) {
    if let Some(node_ref) = root {
        traversal(res, (*node_ref).borrow().left.as_ref());
        res.push((*node_ref).borrow().val);
        traversal(res, (*node_ref).borrow().right.as_ref());
    }
}
