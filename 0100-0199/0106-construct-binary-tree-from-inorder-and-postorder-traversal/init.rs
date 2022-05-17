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

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_tree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        let val = postorder[postorder.len() - 1];
        if let Some((i, _)) = inorder.iter().enumerate().find(|&(_, &v)| v == val) {
            let left = build_tree(&inorder[..i], &postorder[..i]);
            let right = build_tree(&inorder[i + 1..], &postorder[i..postorder.len() - 1]);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        } else {
            let left = None;
            let right = None;
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }
    build_tree(&inorder, &postorder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_ne!(
            build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            None
        );
        assert_eq!(
            build_tree(vec![-1], vec![-1]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: None,
                right: None
            })))
        );
    }
}
