// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn depth(root: Option<Rc<RefCell<TreeNode>>>, balanced: &mut bool) -> i32 {
        if !*balanced {
            return 0;
        }
        if let Some(root) = root {
            let root = root.borrow();
            let left = 1 + depth(root.left.clone(), balanced);
            let right = 1 + depth(root.right.clone(), balanced);
            if i32::abs(left - right) > 1 {
                *balanced = false;
            }
            i32::max(left, right)
        } else {
            0
        }
    }
    let mut balanced = true;
    if let Some(root) = root {
        let root = root.borrow();
        let left = depth(root.left.clone(), &mut balanced);
        let right = depth(root.right.clone(), &mut balanced);
        if i32::abs(left - right) > 1 {
            balanced = false;
        }
    }
    balanced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        is_balanced(None);
    }
}
