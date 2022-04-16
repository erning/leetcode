// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn depth(root: Rc<RefCell<TreeNode>>, d: i32) -> i32 {
        let node = root.borrow();
        match (node.left.clone(), node.right.clone()) {
            (Some(a), Some(b)) => i32::min(depth(a, d + 1), depth(b, d + 1)),
            (Some(a), None) => depth(a, d + 1),
            (None, Some(b)) => depth(b, d + 1),
            (None, None) => d,
        }
    }
    if let Some(root) = root {
        depth(root, 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        min_depth(None);
    }
}
