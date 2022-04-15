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

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || preorder.is_empty() {
            return None;
        }
        let val = preorder[0];
        if let Some((i, _)) = inorder.iter().enumerate().find(|&(_, &v)| v == val) {
            let left = build_tree(&preorder[1..], &inorder[..i]);
            let right = build_tree(&preorder[1 + i..], &inorder[i + 1..]);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        } else {
            let left = None;
            let right = None;
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }
    build_tree(&preorder, &inorder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_ne!(
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
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
