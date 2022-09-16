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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>, 
        root2: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(n1), Some(n2)) =>  {
                let (n1, n2) = (n1.borrow(), n2.borrow());
                let mut root = TreeNode::new(n1.val + n2.val);
                root.left =  Self::merge_trees(n1.left.clone(), n2.left.clone());
                root.right = Self::merge_trees(n1.right.clone(), n2.right.clone());
                Some(Rc::new(RefCell::new(root)))
            }
            (None, Some(n)) | (Some(n), None) => Some(n),
            (None, None) => None,
        }
    }
}
