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
use std::cmp::Ordering;
use std::rc::Rc;

pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, sum: &mut i32) {
        if let Some(node) = root {
            match depth.cmp(max_depth) {
                Ordering::Greater => {
                    *max_depth = depth;
                    *sum = node.borrow().val;
                }
                Ordering::Equal => {
                    *sum += node.borrow().val;
                }
                _ => {}
            }
            // if depth > *max_depth {
            //     *max_depth = depth;
            //     *sum = node.borrow().val;
            // } else if depth == *max_depth {
            //     *sum += node.borrow().val;
            // }
            dfs(node.borrow().left.clone(), depth + 1, max_depth, sum);
            dfs(node.borrow().right.clone(), depth + 1, max_depth, sum);
        }
    }

    let mut sum = 0;
    let mut max_depth = 0;
    dfs(root, 1, &mut max_depth, &mut sum);
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
