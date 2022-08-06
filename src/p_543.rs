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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        dfs(root.as_ref(), &mut ans);
        ans
    }
}

fn dfs(node_opt: Option<&Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
    if let Some(node_ref) = node_opt {
        let node = (**node_ref).borrow();
        let lh = dfs(node.left.as_ref(), ans);
        let rh = dfs(node.right.as_ref(), ans);
        *ans = (*ans).max(lh + rh);
        1 + lh.max(rh)
    } else {
        0
    }
}
