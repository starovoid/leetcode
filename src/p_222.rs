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
    /// The algorithm consists of the following steps:
    ///
    /// 1. Calculate the height of the tree.
    ///
    /// 2. Using a binary search, we are looking for a node of the penultimate layer of the tree that has exactly one leaf.
    /// To perform the search, we match each such node with a number encoding the path to it from the root (bit 0 - turn left, bit 1 - turn right).
    ///
    /// 3. Calculate the number of the outermost leaf and add it to the number of nodes in the intermediate levels of the tree.
    /// If such an intermediate node was not found, we use the property of the binary search algorithm to calculate the number of leaves
    /// using the formula `low * 2 - 1`, where `low` is the left boundary of the search interval at the end of the binary search.
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let height = Self::tree_height(root.clone());
        if height <= 1 {
            return height as i32;
        }

        let mut low = 0;
        let mut high = 2u32.pow(height - 2) - 1;
        let mut rightest_leaf_path = None;

        while low <= high {
            let mid = (low + high) / 2;
            let (count, p) = Self::count_leafs(Rc::clone(root.as_ref().unwrap()), height, mid);
            if count == 1 {
                rightest_leaf_path = Some(p);
                break;
            } else if count == 0 {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        2i32.pow(height - 1) + rightest_leaf_path.unwrap_or(low * 2 - 1) as i32
    }

    /// Find the height of the tree with the given roots by moving along the left.
    fn tree_height(root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
        let mut node_opt = root;
        let mut height = 0;

        while let Some(node) = node_opt {
            height += 1;
            node_opt = node.borrow().left.clone();
        }

        height
    }

    /// Calculate the number of leaves at the node with the specified path and at the same time return the number of the rightest child.
    fn count_leafs(root: Rc<RefCell<TreeNode>>, subtree_height: u32, path: u32) -> (u8, u32) {
        let mut h = subtree_height;
        let mut node = root;
        while h > 2 {
            if path & (1 << (h - 3)) == 0 {
                let left = Rc::clone(&node.borrow().left.as_ref().unwrap());
                node = left;
            } else {
                let right = Rc::clone(&node.borrow().right.as_ref().unwrap());
                node = right
            }
            h -= 1;
        }

        let right_presence = node.borrow().right.is_some() as u8;
        let count = right_presence as u8 + node.borrow().left.is_some() as u8;

        (count, 2 * path + right_presence as u32)
    }
}
