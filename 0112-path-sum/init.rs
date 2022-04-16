// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn path_sum(root: Rc<RefCell<TreeNode>>, sum: i32, found: &mut bool, target: i32) {
        if *found {
            return;
        }
        let node = root.borrow();
        let sum = sum + node.val;
        match (node.left.clone(), node.right.clone()) {
            (Some(a), Some(b)) => {
                path_sum(a, sum, found, target);
                path_sum(b, sum, found, target);
            }
            (Some(a), None) => {
                path_sum(a, sum, found, target);
            }
            (None, Some(b)) => {
                path_sum(b, sum, found, target);
            }
            (None, None) => {
                if sum == target {
                    *found = true;
                }
            }
        }
    }

    let mut found = false;
    if let Some(node) = root {
        path_sum(node, 0, &mut found, target_sum);
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        has_path_sum(None, 0);
    }
}
