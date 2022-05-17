// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let a = 1 + max_depth(node.left.clone());
        let b = 1 + max_depth(node.right.clone());
        i32::max(a, b)
    } else {
        0
    }
}

// func maxDepth(root *TreeNode) int {
//     if root == nil {
//         return 0
//     }
//     d1 := 1 + maxDepth(root.Left)
//     d2 := 1 + maxDepth(root.Right)
//     if d1 > d2 {
//         return d1
//     }
//     return d2
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_depth(None), 0);
    }
}
