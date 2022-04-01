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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        traversal(root.as_ref(), &mut res);
        res
    }
}

fn traversal(node_opt: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        res.push(node.val);
        traversal(node.left.as_ref(), res);
        traversal(node.right.as_ref(), res);
    }
}
