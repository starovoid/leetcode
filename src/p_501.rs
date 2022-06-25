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
use std::collections::HashMap;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut counter: HashMap<i32, usize> = HashMap::new();
        count(root.as_ref(), &mut counter);
        let m = *counter.values().max().unwrap();
        let mut ans = Vec::new();
        for (x, cnt) in counter {
            if cnt == m {
                ans.push(x);
            }
        }
        ans
    }
}

fn count(node_opt: Option<&Rc<RefCell<TreeNode>>>, counter: &mut HashMap<i32, usize>) {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        *counter.entry(node.val).or_insert(0) += 1;
        count(node.left.as_ref(), counter);
        count(node.right.as_ref(), counter);
    }
}
