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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> = vec![(root, 0)];
        while let Some((node_opt, num)) = stack.pop() {
            match node_opt {
                Some(node_ref) => {
                    let node = node_ref.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        sum += num*2 + node.val;
                    } else {
                        stack.push((node.left.as_ref().map(|p| Rc::clone(&p)), num*2 + node.val));
                        stack.push((node.right.as_ref().map(|p| Rc::clone(&p)), num*2 + node.val));
                    }
                }, 
                None => {}
            }
        }
        sum
    }
}
