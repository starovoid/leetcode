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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sorted = Vec::new();
        traverse(root.as_ref(), &mut sorted);
        (0..sorted.len()-1).map(|i| sorted[i+1] - sorted[i]).min().unwrap()
    }
}

fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>) {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        traverse(node.left.as_ref(), s);
        s.push(node.val);
        traverse(node.right.as_ref(), s);
    }
}
