use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        traverse(root.as_ref(), &mut set, k)
    }
}

fn traverse(node_opt: Option<&Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>, k: i32) -> bool {
    if let Some(node_ref) = node_opt {
        let node = (*node_ref).borrow();
        if set.contains(&(k - node.val)) {
            return true;
        }
        set.insert(node.val);
        traverse(node.left.as_ref(), set, k) || traverse(node.right.as_ref(), set, k)
    } else {
        false
    }
}
