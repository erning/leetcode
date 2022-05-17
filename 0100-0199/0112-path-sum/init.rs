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
    fn path_sum(root: Rc<RefCell<TreeNode>>, found: &mut bool, target_sum: i32) {
        if *found {
            return;
        }
        let node = root.borrow();
        let target = target_sum - node.val;
        match (node.left.clone(), node.right.clone()) {
            (Some(a), Some(b)) => {
                path_sum(a, found, target);
                path_sum(b, found, target);
            }
            (Some(a), None) => {
                path_sum(a, found, target);
            }
            (None, Some(b)) => {
                path_sum(b, found, target);
            }
            (None, None) => {
                if target == 0 {
                    *found = true;
                }
            }
        }
    }

    let mut found = false;
    if let Some(node) = root {
        path_sum(node, &mut found, target_sum);
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
