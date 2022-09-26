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
use std::collections::VecDeque;

impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let root = root.unwrap();
        let min: i32 = (*root).borrow().val;
        let mut next_min: i32 = -1;
        deque.push_back(root.borrow().left.clone());
        deque.push_back(root.borrow().right.clone());
        
        while deque.len() > 0 {
            match deque.pop_front().unwrap() {
                None => {},
                Some(node_ref) => {
                    let node = node_ref.borrow();
                    if node.val > min && (node.val < next_min || next_min == -1) {
                        next_min = node.val;
                    }
                    deque.push_back(node.left.clone());
                    deque.push_back(node.right.clone());
                }
            }
        }
        
        next_min
    }
}
