// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32, answer: &mut i32) {
        if *count <= 0 {
            return;
        }
        if let Some(node) = root {
            inorder(node.borrow().left.clone(), count, answer);
            *count -= 1;
            if *count == 0 {
                *answer = node.borrow().val;
            }
            inorder(node.borrow().right.clone(), count, answer);
        }
    }
    let mut count = k;
    let mut answer = 0;
    inorder(root, &mut count, &mut answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        kth_smallest(None, 1);
    }
}
