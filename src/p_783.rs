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
/*
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut numbers: Vec<i32> = Vec::new();
        traverse(root.as_ref(), &mut numbers);
        numbers.iter().zip(numbers.iter().skip(1)).map(|(a, b)| b - a).min().unwrap()
    }
}

fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        traverse(node.left.as_ref(), nums);
        nums.push(node.val);
        traverse(node.right.as_ref(), nums);
    }
}*/

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>, ans: &mut i32, prev: &mut i32) {
            if let Some(node_ref) = node_opt {
                let node = (*node_ref).borrow();
                traverse(node.left.as_ref(), ans, prev);
                if *prev != -1 {
                    *ans = (*ans).min(node.val - *prev);
                }
                *prev = node.val;
                traverse(node.right.as_ref(), ans, prev);
            }
        }
        let mut a = i32::MAX;
        let mut p = -1;
        traverse(root.as_ref(), &mut a, &mut p);
        a
    }
}
