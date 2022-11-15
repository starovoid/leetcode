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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder: Vec<i32> = Vec::new();
        traverse(root.as_ref(), &mut inorder);
        let mut ch = Rc::new(RefCell::new(TreeNode::new(inorder[inorder.len()-1])));
        for val in inorder.into_iter().rev().skip(1) {
            let p = Rc::new(RefCell::new(TreeNode::new(val)));
            p.borrow_mut().right = Some(ch);
            ch = Rc::clone(&p);
        }
        Some(ch)
    }
}

fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>, x: &mut Vec<i32>) {
    if let Some(node_ref) = node_opt {
        let node = node_ref.borrow();
        traverse(node.left.as_ref(), x);
        x.push(node.val);
        traverse(node.right.as_ref(), x);
    }
}
