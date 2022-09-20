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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut values: Vec<Vec<i32>> = Vec::new();
        traverse(root.as_ref(), 0, &mut values);
        values.iter()
            .map(|lvl| lvl.iter().map(|&x| x as f64).sum::<f64>() / (lvl.len() as f64))
            .collect()
    }
}

fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>, lvl: usize, values: &mut Vec<Vec<i32>>) {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        if lvl >= values.len() {
            values.push(vec![node.val]);
        } else {
            values[lvl].push(node.val);
        }
        traverse(node.left.as_ref(), lvl + 1, values);
        traverse(node.right.as_ref(), lvl + 1, values);
    }
}
