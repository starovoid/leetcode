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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        traverse(root.as_ref())
    }
}

fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>) -> String {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        let mut s = String::new();
        s += node.val.to_string().as_str();
        let left_s = traverse(node.left.as_ref());
        let right_s = traverse(node.right.as_ref());
        if right_s != "" {
            s.push('(');
            s += left_s.as_str();
            s.push(')');
            s.push('(');
            s += right_s.as_str();
            s.push(')');
        } else if right_s == "" && left_s != "" {
            s.push('(');
            s += left_s.as_str();
            s.push(')');
        }
        s
    } else {
        String::new()
    }
}
