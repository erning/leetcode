// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, rv: &mut Vec<i32>) {
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            if depth < rv.len() {
                rv[depth] = node.val;
            } else {
                rv.push(node.val);
            }
            preorder(node.left.clone(), depth + 1, rv);
            preorder(node.right.clone(), depth + 1, rv);
        }
    }

    let mut rv = Vec::new();
    preorder(root, 0, &mut rv);
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        right_side_view(None);
    }
}
