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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(0, root.as_ref())
    }
}

fn dfs(depth: i32, node_opt: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = node_opt {
        1 + dfs(depth, (*node).borrow().left.as_ref()).max(
            dfs(depth, (*node).borrow().right.as_ref())
        )
    } else {
        depth
    }
}
