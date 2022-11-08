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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut seq1 = Vec::new();
        let mut seq2 = Vec::new();
        leaf_seq(root1.as_ref(), &mut seq1);
        leaf_seq(root2.as_ref(), &mut seq2);
        seq1 == seq2
    }
}

fn leaf_seq(node_opt: Option<&Rc<RefCell<TreeNode>>>, seq: &mut Vec<i32>) {
    if let Some(node_ref) = node_opt {
        let node = node_ref.borrow();
        if node.left.is_none() && node.right.is_none() {
            seq.push(node.val);
            return;
        }
        leaf_seq(node.left.as_ref(), seq);
        leaf_seq(node.right.as_ref(), seq);
    }
}
