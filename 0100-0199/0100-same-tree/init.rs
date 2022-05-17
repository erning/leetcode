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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            if p.val != q.val {
                return false;
            }
            if !is_same_tree(p.left.clone(), q.left.clone()) {
                return false;
            }
            if !is_same_tree(p.right.clone(), q.right.clone()) {
                return false;
            }
            true
        }
        (Some(_), None) | (None, Some(_)) => false,
        (None, None) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_same_tree(None, None), true);
    }
}
