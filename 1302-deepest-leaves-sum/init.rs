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

pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum = 0;
    if let Some(node) = root {
        let mut currs: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        currs.push(node);
        while !currs.is_empty() {
            sum = 0;
            let mut nexts: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            for node in currs.into_iter() {
                sum += node.borrow().val;
                if let Some(next) = node.borrow().left.clone() {
                    nexts.push(next);
                }
                if let Some(next) = node.borrow().right.clone() {
                    nexts.push(next);
                }
            }
            currs = nexts;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        deepest_leaves_sum(None);
    }
}
