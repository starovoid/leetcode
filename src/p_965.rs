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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let r = root.unwrap();
        let v = r.borrow().val;
        solve(Some(&r), v)
    }
}

fn solve(node_opt: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> bool {
    match node_opt {
        None => true,
        Some(node_ref) => {
            let node = node_ref.borrow();
            if node.val != val {
                return false;
            }
            solve(node.left.as_ref(), val) && solve(node.right.as_ref(), val)
        }
    }
}
