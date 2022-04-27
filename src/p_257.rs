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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = Vec::new();
        gen(root.as_ref(), vec![(**root.as_ref().unwrap()).borrow().val], &mut ans);
        ans
    }
}

fn gen(node_opt: Option<&Rc<RefCell<TreeNode>>>, path: Vec<i32>, ans: &mut Vec<String>) {
    if let Some(node_ref) = node_opt {
        let node = (**node_ref).borrow();
        if node.left.is_some() {
            let mut p = path.clone();
            p.push((**node.left.as_ref().unwrap()).borrow().val);
            gen(node.left.as_ref(), p, ans);
        }
        if node.right.is_some() {
            let mut p = path.clone();
            p.push((**node.right.as_ref().unwrap()).borrow().val);
            gen(node.right.as_ref(), p, ans);
        }
        if node.left.is_none() && node.right.is_none() {
            ans.push(path.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join("->"));
        }
    }
}
